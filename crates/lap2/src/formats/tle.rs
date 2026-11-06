#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SatelliteName([u8; 24]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TleLine([u8; 69]);

impl TleLine {
    /// Expected length of the TLE line in bytes.
    pub const LEN: usize = 69;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TleLine1(TleLine);

impl TleLine1 {
    /// Expected line prefix ("1 ").
    pub const PREFIX: &[u8; 2] = b"1 ";

    /// Expected length of the TLE line in bytes.
    pub const LEN: usize = TleLine::LEN;

    /// Index where ' ' is located.
    pub const SPACE_INDICES: [usize; 7] = [8, 17, 32, 43, 52, 61, 63];

    /// Index where '.' is located.
    pub const DOT_INDICES: [usize; 2] = [23, 34];

    /// Index where either '+', '-' or ' ' is placed.
    pub const SIGN_OR_SPACE_INDICES: [usize; 2] = [33, 44];

    /// Index where the sign of the exponent is placed.
    /// If ‘+’ or ‘-’ is not present here, this index and the next one will be left blank.
    pub const EXPONENT_INDICES: [usize; 2] = [50, 59];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TleLine2(TleLine);

impl TleLine2 {
    /// Expected line prefix ("2 ").
    pub const PREFIX: &[u8; 2] = b"2 ";

    /// Expected length of the TLE line in bytes.
    pub const LEN: usize = TleLine::LEN;

    /// Index where ' ' is located.
    pub const SPACE_INDICES: [usize; 6] = [7, 16, 25, 33, 42, 51];

    /// Index where '.' is located.
    pub const DOT_INDICES: [usize; 5] = [11, 20, 37, 46, 54];
}

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
