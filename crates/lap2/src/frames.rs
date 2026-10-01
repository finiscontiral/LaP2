use std::fmt;

use nalgebra::Vector3;
use satkit::Instant;
use uom::si::{
    angle::degree,
    f64::{Angle, Length},
};

use crate::{Error, error::FrameError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrameType {
    Gcef,
    Itrf,
    LLA,
}

impl fmt::Display for FrameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gcef => write!(f, "GCEF"),
            Self::Itrf => write!(f, "ITRF"),
            Self::LLA => write!(f, "LLA"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    Cartesian(CartesianPosition),
    /// 測地座標系（緯度, 経度, 高度）
    Lla(LlaPosition),
}

/// Cartesian Position
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CartesianPosition {
    pub vector: Vector3<Length>,
}

impl CartesianPosition {
    pub const fn new(vector: Vector3<Length>) -> Self {
        Self { vector }
    }
}

/// Geodetic coordinate
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LlaPosition {
    pub lat: Angle,
    pub lon: Angle,
    pub alt: Length,
}

impl LlaPosition {
    pub fn try_new(lat: Angle, lon: Angle, alt: Length) -> Result<Self, Error> {
        let lat_deg = lat.get::<degree>();
        let lon_deg = lon.get::<degree>();

        if !(-90.0..=90.0).contains(&lat_deg) {
            return Err(Error::Frame(FrameError::LatitudeOutOfRange(lat_deg)));
        }
        if !(-180.0..=180.0).contains(&lon_deg) {
            return Err(Error::Frame(FrameError::LongitudeOutOfRange(lon_deg)));
        }

        Ok(Self { lat, lon, alt })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    pub time: Instant,
    pub position: Position,
    pub velocity: Option<Vector3<Length>>,
    pub frame: FrameType,
}

impl Frame {
    pub fn try_new(
        time: Instant,
        position: Position,
        velocity: Option<Vector3<Length>>,
        frame: FrameType,
    ) -> Result<Self, &'static str> {
        match (frame, &position) {
            (FrameType::LLA, Position::Cartesian(_)) => {
                return Err("LLA frame type requires LLA position coordinates.");
            }
            (ft, Position::Lla { .. }) if ft != FrameType::LLA => {
                return Err("Cartesian frame types require Cartesian position coordinates.");
            }
            _ => {}
        }

        Ok(Self {
            time,
            position,
            velocity,
            frame,
        })
    }

    pub fn try_from_lla(time: Instant, lat: Angle, lon: Angle, alt: Length) -> Result<Self, Error> {
        let position = LlaPosition::try_new(lat, lon, alt)?;
        Ok(Self {
            time,
            position: Position::Lla(position),
            velocity: None,
            frame: FrameType::LLA,
        })
    }

    pub fn from_cartesian(
        time: Instant,
        position: Vector3<Length>,
        velocity: Option<Vector3<Length>>,
        frame: FrameType,
    ) -> Result<Self, &'static str> {
        if frame == FrameType::LLA {
            return Err("Use from_lla for LLA frames.");
        }

        let _position = CartesianPosition::new(position);

        Ok(Self {
            time,
            position: Position::Cartesian(_position),
            velocity,
            frame,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod lla {
        use super::*;
        use uom::si::length::meter;

        #[test]
        fn test_valid() {
            let lat = Angle::new::<degree>(35.0);
            let lon = Angle::new::<degree>(135.0);
            let alt = Length::new::<meter>(10.0);

            let result = LlaPosition::try_new(lat, lon, alt);
            assert!(result.is_ok(), "Valid LLA position should succeed");
        }

        #[test]
        fn test_boundary() {
            let lat_north = Angle::new::<degree>(90.0);
            let lat_south = Angle::new::<degree>(-90.0);
            let lon = Angle::new::<degree>(0.0);
            let alt = Length::new::<meter>(0.0);

            assert!(LlaPosition::try_new(lat_north, lon, alt).is_ok());
            assert!(LlaPosition::try_new(lat_south, lon, alt).is_ok());
        }

        #[test]
        fn test_invalid_latitude() {
            let lat_invalid = Angle::new::<degree>(91.0);
            let lon = Angle::new::<degree>(0.0);
            let alt = Length::new::<meter>(0.0);

            let result = LlaPosition::try_new(lat_invalid, lon, alt);
            assert!(result.is_err(), "Latitude > 90 should fail");
        }
    }
}
