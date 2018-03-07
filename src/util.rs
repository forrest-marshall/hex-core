//! non-essential utilities.
//!
use hex;


/// equivalent to `hex::into` with additional `&str` return.
///
/// this function is useful for applications which require temporary use
/// of a hexadecimal string, such as for use with 
/// [`std::fmt::Write::write_str`](https://doc.rust-lang.org/std/fmt/trait.Write.html#tymethod.write_str)
///
#[inline]
pub fn as_str<'s,'d>(src: &'s [u8], dst: &'d mut [u8]) -> &'d str {
    hex::into(src,dst);
    debug_assert!(dst.is_ascii(),"must produce valid ascii");
    unsafe { ::std::str::from_utf8_unchecked(dst) } 
}


/// equivalent to `hex::into_upper` with additional `&str` return.
/// 
/// this function is useful for applications which require temporary use
/// of a hexadecimal string, such as for use with 
/// [`std::fmt::Write::write_str`](https://doc.rust-lang.org/std/fmt/trait.Write.html#tymethod.write_str)
///
#[inline]
pub fn as_str_upper<'s,'d>(src: &'s [u8], dst: &'d mut [u8]) -> &'d str {
    hex::into_upper(src,dst);
    debug_assert!(dst.is_ascii(),"must produce valid ascii");
    unsafe { ::std::str::from_utf8_unchecked(dst) }
}



/// check if supplied byte is a valid hexadecimal byte (UTF-8).
///
#[inline]
pub fn is_hex_byte(byte: u8) -> bool {
    match byte {
        b'A'...b'F' => true,
        b'a'...b'f' => true,
        b'0'...b'9' => true,
        _ => false,
    }
}



/// check if supplied slice is valid hexadecimal bytes (UTF-8).
///
#[inline]
pub fn is_hex_slice(bytes: &[u8]) -> bool {
    for byte in bytes.iter() {
        if !is_hex_byte(*byte) { return false; }
    }
    true
}


#[cfg(test)]
mod tests {
    use util;

    const LOWER: &'static str = "0123456789abcdef";
    
    const UPPER: &'static str = "0123456789ABCDEF";
    
    const BYTES: &'static [u8] = &[
        0x01,0x23,0x45,0x67,0x89,0xab,0xcd,0xef
    ];


    #[test]
    fn lower() {
        let mut lower_buff = vec![0u8;LOWER.len()];
        let lower_str = util::as_str(BYTES,&mut lower_buff);
        assert_eq!(LOWER,lower_str);
    }


    #[test]
    fn upper() {
        let mut upper_buff = vec![0u8;UPPER.len()];
        let upper_str = util::as_str_upper(BYTES,&mut upper_buff);
        assert_eq!(UPPER,upper_str);
    }

    #[test]
    fn bytes() {
        let upper: &[u8] = UPPER.as_ref();
        let lower: &[u8] = LOWER.as_ref();
        for byte in lower.iter().chain(upper) {
            assert!(util::is_hex_byte(*byte));
        }
        let nonhex = (0..256u32).filter_map(|i| {
            let byte = i as u8;
            if lower.contains(&byte) || upper.contains(&byte) {
                None
            } else {
                Some(byte)
            }
        });
        for byte in nonhex {
            assert!(!util::is_hex_byte(byte));
        }
    }

    #[test]
    fn slice() {
        let upper: &[u8] = UPPER.as_ref();
        let lower: &[u8] = LOWER.as_ref(); 
        assert!(util::is_hex_slice(upper));
        assert!(util::is_hex_slice(lower));
        assert!(util::is_hex_slice(&[]));
        let nonhex = (0..256u32).filter_map(|i| {
            let byte = i as u8;
            if lower.contains(&byte) || upper.contains(&byte) {
                None
            } else {
                Some(byte)
            }
        });
        for byte in nonhex {
            assert!(!util::is_hex_slice(&[byte]));
        }
    }
}

