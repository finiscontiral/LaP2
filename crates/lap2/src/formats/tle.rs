use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, FromBytes, IntoBytes, Immutable, KnownLayout, Unaligned,
)]
#[repr(transparent)]
pub struct SatelliteName([u8; 24]);

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, FromBytes, IntoBytes, Immutable, KnownLayout, Unaligned,
)]
#[repr(transparent)]
struct TleLine([u8; 69]);

impl TleLine {
    /// Expected length of the TLE line in bytes.
    pub const LEN: usize = 69;

    pub(crate) fn validate_checksum(line: &[u8; Self::LEN]) -> Result<(), ()> {
        let last_byte = line[Self::LEN - 1];

        if !last_byte.is_ascii_digit() {
            todo!()
        }

        let expected = last_byte - b'0';
        let actual = compute_tle_checksum(&line[..line.len() - 1]);

        if actual != expected {
            todo!()
        }

        Ok(())
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, FromBytes, IntoBytes, Immutable, KnownLayout, Unaligned,
)]
#[repr(transparent)]
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
    pub const SIGN_OR_SPACE_INDICES: [usize; 3] = [33, 44, 53];

    /// Index where the sign of the exponent is placed.
    /// If ‘+’ or ‘-’ is not present here, this index and the next one will be left blank.
    pub const EXPONENT_INDICES: [usize; 2] = [50, 59];

    pub fn parse(source: &[u8]) -> Result<&Self, ()> {
        let line1 = TleLine1::ref_from_bytes(source).map_err(|_| ())?;
        Self::validate(&line1.0.0)?;

        Ok(line1)
    }

    pub(crate) fn validate(line: &[u8; Self::LEN]) -> Result<(), ()> {
        if !line.starts_with(Self::PREFIX) {
            todo!()
        };

        check_expected_char(line, &Self::SPACE_INDICES, b' ').map_err(|idx| ())?;

        check_expected_char(line, &Self::DOT_INDICES, b'.').map_err(|idx| ())?;

        check_any_char(line, &Self::SIGN_OR_SPACE_INDICES, &[b' ', b'+', b'-'])
            .map_err(|idx| ())?;

        for idx in Self::EXPONENT_INDICES {
            match line[idx] {
                b'+' | b'-' => {
                    if !line[idx + 1].is_ascii_digit() {
                        todo!()
                    }
                }
                b' ' => {
                    if line[idx + 1] != b' ' {
                        todo!()
                    }
                }
                _ => todo!(),
            }
        }

        TleLine::validate_checksum(line)?;

        Ok(())
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, FromBytes, IntoBytes, Immutable, KnownLayout, Unaligned,
)]
#[repr(transparent)]
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

    pub fn parse(source: &[u8]) -> Result<&Self, ()> {
        let line2 = TleLine2::ref_from_bytes(source).map_err(|_| ())?;
        Self::validate(&line2.0.0)?;

        Ok(line2)
    }

    pub(crate) fn validate(line: &[u8; Self::LEN]) -> Result<(), ()> {
        if !line.starts_with(Self::PREFIX) {
            todo!()
        };

        check_expected_char(line, &Self::SPACE_INDICES, b' ').map_err(|idx| ())?;

        check_expected_char(line, &Self::DOT_INDICES, b'.').map_err(|idx| ())?;

        TleLine::validate_checksum(line)?;

        Ok(())
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, FromBytes, IntoBytes, Immutable, KnownLayout, Unaligned,
)]
#[repr(C)]
pub struct Tle {
    pub line1: TleLine1,
    pub line2: TleLine2,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, FromBytes, IntoBytes, Immutable, KnownLayout, Unaligned,
)]
#[repr(C)]
pub struct NameTle {
    pub name: SatelliteName,
    pub tle: Tle,
}

fn compute_tle_checksum(bytes: &[u8]) -> u8 {
    let sum: u32 = bytes
        .iter()
        .map(|b| match b {
            b'0'..=b'9' => (b - b'0') as u32,
            b'-' => 1,
            _ => 0,
        })
        .sum();

    (sum % 10) as u8
}

/// `bytes`内に指定した`expected`が`indices`の位置にあるか確認する関数
fn check_expected_char(bytes: &[u8], indices: &[usize], expected: u8) -> Result<(), usize> {
    for &idx in indices {
        if bytes[idx] != expected {
            return Err(idx);
        }
    }
    Ok(())
}

/// `bytes`内に指定した`expected_chars`が`indices`の位置にあるか確認する関数
fn check_any_char(bytes: &[u8], indices: &[usize], expected_chars: &[u8]) -> Result<(), usize> {
    for &idx in indices {
        if !expected_chars.contains(&bytes[idx]) {
            return Err(idx);
        }
    }
    Ok(())
}
