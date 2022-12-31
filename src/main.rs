use std::{time::Duration, process::exit};

use discord_rich_presence::{DiscordIpcClient, DiscordIpc, activity::{self, Activity, Timestamps, Assets, Button}};
use iceportal::{ICEPortal, trip_info::TripInfo, global_models::{PositionStatus, Stop}};
use inquire::Select;
use tokio::{select, time::sleep};

fn cancel_activity(client: &mut DiscordIpcClient) {
    client.clear_activity().expect("Error while clearing activity");
}

fn update_activity(client: &mut DiscordIpcClient, trip: TripInfo, to: &str, building_series: &str) {
    let end_stop = trip.stops.iter().find(|stop| stop.station.name == to).unwrap();
    if end_stop.info.position_status.is_some() && end_stop.info.position_status != Some(PositionStatus::Future) {
        println!("Welcome in {}", end_stop.station.name);
        cancel_activity(client);
        exit(0);
    }
    let next_stop = trip.stops.iter()
        .filter(|stop| stop.info.position_status == Some(PositionStatus::Future))
        .collect::<Vec<&Stop>>()[0];

    let timestamps = Timestamps::new()
        .end(end_stop.timetable.actual_arrival_time.unwrap().timestamp());
    let assets = Assets::new().large_image(building_series);
    let watch_button_url = format!("https://regenbogen-ice.de/trip/{}/{}", trip.train_type, trip.vzn);
    let watch_button = Button::new("Watch", 
        watch_button_url.as_str());

    let details = format!("Riding {} {} to {}", trip.train_type, trip.vzn, to);
    let state = format!("Next stop: {}", next_stop.station.name);
    let activity = Activity::new()
        .details(details.as_str())
        .state(state.as_str())
        .timestamps(timestamps)
        .assets(assets)
        .buttons(vec![watch_button]);
    client.set_activity(activity)
        .expect("Error while setting new activity");
}

#[tokio::main]
async fn main() {
    let trip_info = ICEPortal::fetch_trip_info().await
        .expect("Error while fetching iceportal data");
    let status_info = ICEPortal::fetch_status().await
        .expect("Error while fetching iceportal data");
    let available_stops = trip_info.trip.stops.iter()
        .map(|stop| stop.station.name.as_str()).collect::<Vec<&str>>();
    let leave_station = Select::new("At which station will you leave the train?", available_stops).prompt()
        .expect("Error while prompt");
    
    let (cancel_sender, mut cancel) = tokio::sync::oneshot::channel();
    tokio::spawn(async {
        tokio::signal::ctrl_c().await.unwrap();
        cancel_sender.send(()).unwrap();
    });

    let (trip_info_sender, mut trip_info_receiver) = tokio::sync::mpsc::unbounded_channel();
    tokio::spawn(async move {
        loop {
            let trip_info = ICEPortal::fetch_trip_info().await.expect("Error while fetching trip info");
            trip_info_sender.send(trip_info)
                .expect("Error while putting message to trip_info channel");
            sleep(Duration::from_secs(30)).await;
        }
    });
    println!("Connecting to discord ipc...");

    let mut client = DiscordIpcClient::new("1058750299675824128")
        .expect("Error while creating discord ipc client");
    client.connect()
        .expect("Error while connecting ipc client");
    
    let payload = activity::Activity::new().state("This is a state");
    client.set_activity(payload)
        .expect("Error while setting activity");
    
    println!("Connected! ICEPortal rich presense is running. Stop it by pressing Ctrl + C");

    loop {
        select! {
            Some(trip_info) = trip_info_receiver.recv() => {
                update_activity(&mut client, trip_info.trip, leave_station, status_info.series.as_str());
            },
            _ = &mut cancel => {
                println!("Received Ctrl + C");
                cancel_activity(&mut client);
                break;
            },
        }
    }
}
