//! non-essential utilities.
//!
use hex;


/// equivalent to `hex::into` with additional `&str` return.
///
pub fn as_str<'s,'d>(src: &'s [u8], dst: &'d mut [u8]) -> &'d str {
    hex::into(src,dst);
    debug_assert!(dst.is_ascii(),"must produce valid ascii");
    unsafe { ::std::str::from_utf8_unchecked(dst) } 
}


/// equivalent to `hex::into_upper` with additional `&str` return.
///
pub fn as_str_upper<'s,'d>(src: &'s [u8], dst: &'d mut [u8]) -> &'d str {
    hex::into_upper(src,dst);
    debug_assert!(dst.is_ascii(),"must produce valid ascii");
    unsafe { ::std::str::from_utf8_unchecked(dst) }
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
}

