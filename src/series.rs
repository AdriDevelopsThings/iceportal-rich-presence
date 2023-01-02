pub fn translate_series(series: &str) -> &str {
    let series_as_int = series.parse::<u16>();
    match series_as_int {
        Ok(s) => {
            if (801..804).contains(&s) {
                "401"
            } else if (805..808).contains(&s) {
                "402"
            } else if s == 812 {
                "412"
            } else {
                series
            }
        },
        Err(_) => series
    }

}