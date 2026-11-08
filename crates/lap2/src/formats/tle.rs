use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::{Error, error::TleError};

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

    pub(crate) fn validate_checksum(line: &[u8; Self::LEN]) -> Result<(), Error> {
        let last_byte = line[Self::LEN - 1];

        if !last_byte.is_ascii_digit() {
            return Err(TleError::UnexpectedCharacter {
                idx: Self::LEN - 1,
                found: last_byte as char,
            })?;
        }

        let expected = last_byte - b'0';
        let actual = compute_tle_checksum(&line[..line.len() - 1]);

        if actual != expected {
            return Err(TleError::InvalidChecksum { expected, actual })?;
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

    pub fn parse(source: &[u8]) -> Result<&Self, Error> {
        let line1 = TleLine1::ref_from_bytes(source).map_err(|_| TleError::InvalidLength)?;
        Self::validate(&line1.0.0)?;

        Ok(line1)
    }

    pub(crate) fn validate(line: &[u8; Self::LEN]) -> Result<(), Error> {
        if !line.starts_with(Self::PREFIX) {
            return Err(TleError::InvalidPrefix {
                expected: Self::PREFIX,
            })?;
        };

        check_expected_char(line, &Self::SPACE_INDICES, b' ').map_err(|idx| {
            TleError::InvalidCharacter {
                idx,
                expected: &[' '],
                found: line[idx] as char,
            }
        })?;

        check_expected_char(line, &Self::DOT_INDICES, b'.').map_err(|idx| {
            TleError::InvalidCharacter {
                idx,
                expected: &['.'],
                found: line[idx] as char,
            }
        })?;

        check_any_char(line, &Self::SIGN_OR_SPACE_INDICES, &[b' ', b'+', b'-']).map_err(|idx| {
            TleError::InvalidCharacter {
                idx,
                expected: &[' ', '+', '-'],
                found: line[idx] as char,
            }
        })?;

        for idx in Self::EXPONENT_INDICES {
            match line[idx] {
                b'+' | b'-' => {
                    if !line[idx + 1].is_ascii_digit() {
                        return Err(TleError::UnexpectedCharacter {
                            idx: idx + 1,
                            found: line[idx + 1] as char,
                        })?;
                    }
                }
                b' ' => {
                    if line[idx + 1] != b' ' {
                        return Err(TleError::InvalidCharacter {
                            idx: idx + 1,
                            expected: &[' '],
                            found: line[idx + 1] as char,
                        })?;
                    }
                }
                _ => Err(TleError::InvalidCharacter {
                    idx,
                    expected: &[' ', '+', '-'],
                    found: line[idx] as char,
                })?,
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

    pub fn parse(source: &[u8]) -> Result<&Self, Error> {
        let line2 = TleLine2::ref_from_bytes(source).map_err(|_| TleError::InvalidLength)?;
        Self::validate(&line2.0.0)?;

        Ok(line2)
    }

    pub(crate) fn validate(line: &[u8; Self::LEN]) -> Result<(), Error> {
        if !line.starts_with(Self::PREFIX) {
            return Err(TleError::InvalidPrefix {
                expected: Self::PREFIX,
            })?;
        };

        check_expected_char(line, &Self::SPACE_INDICES, b' ').map_err(|idx| {
            TleError::InvalidCharacter {
                idx,
                expected: &[' '],
                found: line[idx] as char,
            }
        })?;

        check_expected_char(line, &Self::DOT_INDICES, b'.').map_err(|idx| {
            TleError::InvalidCharacter {
                idx,
                expected: &['.'],
                found: line[idx] as char,
            }
        })?;

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

#[cfg(test)]
mod tests {
    use super::*;

    /// Test TLE sample.
    /// Source [SPACE-TRACK.ORG](https://www.space-track.org/documentation#/tle)
    /// TLE data listed in the Basic Info section.
    const SAMPLE_TLE_NAME: &str = "ISS (ZARYA)";
    const SAMPLE_TLE_LINE1: &[u8; 69] =
        b"1 25544U 98067A   04236.56031392  .00020137  00000-0  16538-3 0  9993";
    const SAMPLE_TLE_LINE2: &[u8; 69] =
        b"2 25544  51.6335 344.7760 0007976 126.2523 325.9359 15.70406856328906";

    mod line1 {
        use super::{SAMPLE_TLE_LINE1, TleLine1};

        #[test]
        fn test_constants() {
            assert_eq!(TleLine1::PREFIX, b"1 ");
            assert_eq!(TleLine1::LEN, 69);
            assert_eq!(TleLine1::SPACE_INDICES, [8, 17, 32, 43, 52, 61, 63]);
            assert_eq!(TleLine1::DOT_INDICES, [23, 34]);
            assert_eq!(TleLine1::SIGN_OR_SPACE_INDICES, [33, 44, 53]);
            assert_eq!(TleLine1::EXPONENT_INDICES, [50, 59]);

            for i in TleLine1::SPACE_INDICES {
                assert!(i < TleLine1::LEN)
            }
            for i in TleLine1::DOT_INDICES {
                assert!(i < TleLine1::LEN)
            }
            for i in TleLine1::SIGN_OR_SPACE_INDICES {
                assert!(i < TleLine1::LEN)
            }
            for i in TleLine1::EXPONENT_INDICES {
                assert!(i < TleLine1::LEN);
                assert!(i + 1 < TleLine1::LEN)
            }
        }

        #[test]
        fn test_validate() {
            let result = TleLine1::validate(SAMPLE_TLE_LINE1);
            assert!(
                result.is_ok(),
                "Failed to validate line1: {:?}",
                result.err()
            );
        }
    }

    mod line2 {
        use super::{SAMPLE_TLE_LINE2, TleLine2};

        #[test]
        fn test_constants() {
            assert_eq!(TleLine2::PREFIX, b"2 ");
            assert_eq!(TleLine2::LEN, 69);
            assert_eq!(TleLine2::SPACE_INDICES, [7, 16, 25, 33, 42, 51]);
            assert_eq!(TleLine2::DOT_INDICES, [11, 20, 37, 46, 54]);

            for i in TleLine2::SPACE_INDICES {
                assert!(i < TleLine2::LEN)
            }
            for i in TleLine2::DOT_INDICES {
                assert!(i < TleLine2::LEN)
            }
        }

        #[test]
        fn test_validate() {
            let result = TleLine2::validate(SAMPLE_TLE_LINE2);
            assert!(
                result.is_ok(),
                "Failed to validate line2: {:?}",
                result.err()
            );
        }
    }

    mod compute_tle_checksum {
        use super::*;

        #[test]
        fn test_empty() {
            let bytes = b"";
            let checksum = compute_tle_checksum(bytes);
            assert_eq!(checksum, 0);
        }

        #[test]
        fn test_space() {
            let bytes = b" ";
            let checksum = compute_tle_checksum(bytes);
            assert_eq!(checksum, 0);
        }

        #[test]
        fn test_plus() {
            let bytes = b"+";
            let checksum = compute_tle_checksum(bytes);
            assert_eq!(checksum, 0);
        }

        #[test]
        fn test_minus() {
            let bytes = b"-";
            let checksum = compute_tle_checksum(bytes);
            assert_eq!(checksum, 1);
        }

        #[test]
        fn test_digits() {
            let bytes = b"0123456789";
            let checksum = compute_tle_checksum(bytes);
            assert_eq!(checksum, 5);
        }

        #[test]
        fn test_alphabet() {
            let bytes = b"qwertyuiopasdfghjklzxcvbnm";
            let checksum = compute_tle_checksum(bytes);
            assert_eq!(checksum, 0);
        }

        #[test]
        fn test_exact_ten() {
            // '5' (5) + '5' (5) = 10 -> 10 % 10 = 0
            let bytes = b"55";
            let checksum = compute_tle_checksum(bytes);
            assert_eq!(checksum, 0);
        }

        #[test]
        fn test_modulo_ten() {
            // '9' (9) + '9' (9) = 18 -> 18 % 10 = 8
            let bytes = b"99";
            let checksum = compute_tle_checksum(bytes);
            assert_eq!(checksum, 8);
        }
    }
}
