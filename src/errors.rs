use std::fmt::Debug;

use iceportal::errors::ICEPortalError;

pub enum ICEPortalRichPresenseError {
    ICEPortalError(ICEPortalError)
}

impl From<ICEPortalError> for ICEPortalRichPresenseError {
    fn from(e: ICEPortalError) -> Self {
        Self::ICEPortalError(e)
    }
}

impl Debug for ICEPortalRichPresenseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ICEPortalError(_) => 
                write!(f, "Error while connecting to ICE portal: Please check if you are connected to the Bahn WiFi")
        }
    }
}