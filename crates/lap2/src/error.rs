use thiserror::Error;

use crate::frames::FrameType;

#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum Error {
    #[error(transparent)]
    Frame(#[from] FrameError),

    #[error(transparent)]
    Tle(#[from] TleError),
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

#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum TleError {
    #[error("invalid length: insufficient bytes for TLE line")]
    InvalidLength,

    #[error("invalid prefix: TLE line{expected:?}must start with '{expected:?}'")]
    InvalidPrefix { expected: &'static [u8] },

    #[error("invalid character at index {idx}: expected '{expected:?}', found '{found}'")]
    InvalidCharacter {
        idx: usize,
        expected: &'static [char],
        found: char,
    },

    #[error("invalid character at index {idx}: unexpected character '{found}'")]
    UnexpectedCharacter { idx: usize, found: char },

    #[error("invalid checksum: expected {expected}, got {actual}")]
    InvalidChecksum { expected: u8, actual: u8 },
}
