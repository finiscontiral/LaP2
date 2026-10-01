use thiserror::Error;

use crate::frames::FrameType;

#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum Error {
    #[error(transparent)]
    Frame(#[from] FrameError),
}

#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum FrameError {
    #[error("Latitude must be between -90 and 90 degrees (got {0})")]
    LatitudeOutOfRange(f64),

    #[error("Longitude must be between -180 and 180 degrees (got {0})")]
    LongitudeOutOfRange(f64),

    #[error("Unsupported frame type: {0}")]
    UnsupportedFrameType(FrameType),
}
