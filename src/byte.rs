//! single-byte hexadecimal conversions.
//!
use error::ParseHexError;


/// attempt to parse the value of a byte encoded as a pair of
/// hexadecimal bytes.
#[inline]
pub fn from(char_a: u8, char_b: u8) -> Result<u8,ParseHexError> {
    let byte_a = match char_a {
        val @ b'A'...b'F' => val - b'A' + 10,
        val @ b'a'...b'f' => val - b'a' + 10,
        val @ b'0'...b'9' => val - b'0',
        _ => return Err(ParseHexError::NonHexByte),
    };
    let byte_b = match char_b {
        val @ b'A'...b'F' => val - b'A' + 10,
        val @ b'a'...b'f' => val - b'a' + 10,
        val @ b'0'...b'9' => val - b'0',
        _ => return Err(ParseHexError::NonHexByte),
    };
    let byte = byte_a << 4 | byte_b & 0x0f;
    Ok(byte)
}


/// convert a standard byte value into a pair of hexadecimal bytes.
///
#[inline]
pub fn into(byte: u8) -> (u8,u8) {
    let (byte_a, byte_b) = ( byte >> 4, byte & 0x0f );
    let char_a = match byte_a {
        val @ 0xa...0xf => val - 0xa + b'a',
        val @ 0x0...0x9 => val + b'0',
        _ => unreachable!("value never outside range 0x0...0xf")
    };
    let char_b = match byte_b {
        val @ 0xa...0xf => val - 0xa + b'a',
        val @ 0x0...0x9 => val + b'0',
        _ => unreachable!("value never outside range 0x0...0xf")
    };
    (char_a,char_b)
}


/// convert a standard byte value into a pair of uppercase hexadecimal bytes.
///
#[inline]
pub fn into_upper(byte: u8) -> (u8,u8) {
    let (byte_a, byte_b) = ( byte >> 4, byte & 0x0f );
    let char_a = match byte_a {
        val @ 0xa...0xf => val - 0xa + b'A',
        val @ 0x0...0x9 => val + b'0',
        _ => unreachable!("value never outside range 0x0...0xf")
    };
    let char_b = match byte_b {
        val @ 0xa...0xf => val - 0xa + b'A',
        val @ 0x0...0x9 => val + b'0',
        _ => unreachable!("value never outside range 0x0...0xf")
    };
    (char_a,char_b)
}


#[cfg(test)]
mod tests {
    use byte;

    #[test]
    fn lower() {
        for i in 0..256u32 {
            let i = i as u8;
            let (a,b) = byte::into(i);
            assert_eq!(i,byte::from(a,b).unwrap());
        }
    }

    #[test]
    fn upper() {
        for i in 0..256u32 {
            let i = i as u8;
            let (a,b) = byte::into_upper(i);
            assert_eq!(i,byte::from(a,b).unwrap());
        }
    }
}
