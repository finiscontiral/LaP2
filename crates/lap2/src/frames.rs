use nalgebra::Vector3;
use satkit::Instant;
use uom::si::{
    angle::degree,
    f64::{Angle, Length},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrameType {
    Gcef,
    Itrf,
    LLA,
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
    pub fn try_new(lat: Angle, lon: Angle, alt: Length) -> Result<Self, ()> {
        let lat_deg = lat.get::<degree>();
        let lon_deg = lon.get::<degree>();

        if !(-90.0..=90.0).contains(&lat_deg) {
            panic!("Latitude must be between -90 and 90 degrees.");
        }
        if !(-180.0..=180.0).contains(&lon_deg) {
            panic!("Longitude must be between -180 and 180 degrees.");
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

    pub fn try_from_lla(time: Instant, lat: Angle, lon: Angle, alt: Length) -> Result<Self, ()> {
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
