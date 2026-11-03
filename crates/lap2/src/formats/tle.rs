#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SatelliteName([u8; 24]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TleLine([u8; 69]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TleLine1(TleLine);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TleLine2(TleLine);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tle {
    pub line1: TleLine1,
    pub line2: TleLine2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NameTle {
    pub name: SatelliteName,
    pub tle: Tle,
}
