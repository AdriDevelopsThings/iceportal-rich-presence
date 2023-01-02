use std::fmt::Debug;

use iceportal::errors::ICEPortalError;

pub enum ICEPortalRichPresenceError {
    ICEPortalError(ICEPortalError)
}

impl From<ICEPortalError> for ICEPortalRichPresenceError {
    fn from(e: ICEPortalError) -> Self {
        Self::ICEPortalError(e)
    }
}

impl Debug for ICEPortalRichPresenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ICEPortalError(e) => {
                match e {
                    ICEPortalError::RequestError(_) =>
                        write!(f, "It seems like a connection error to te ICE Portal happend: {:?}", e),
                    ICEPortalError::NotConnectedToICEPortal =>
                        write!(f, "Error while connecting to ICE portal: Please check if you are connected to the Bahn WiFi")
                }
            }
        }
    }
}