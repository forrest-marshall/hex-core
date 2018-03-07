//! primary hexadecimal conversion functions.
//!
use error::ParseHexError;
use byte;


/// convert from hexadecimal to byte values.
///
/// parses a slice of hexadecimal bytes, writing output to the supplied buffer.
///
/// # panics
///
/// this function will panic if the `src` buffer is not exactly twice the size 
/// of the `dst` buffer.
///
#[inline]
pub fn from(src: &[u8], dst: &mut [u8]) -> Result<(),ParseHexError> {
    assert_eq!(src.len(),dst.len() * 2,"hex buff must be 2x the size of byte buff");
    for (src_chunk,dst_byte) in src.chunks(2).zip(dst.iter_mut()) {
        debug_assert!(src_chunk.len() == 2,"chunks should always be of length 2");
        *dst_byte = byte::from(src_chunk[0],src_chunk[1])?;
    }
    Ok(())
}


/// convert from byte values to hexadecimal.
///
/// process a slice of byte values, writing their corresponding hexadecimal 
/// representation to the supplied buffer.
///
/// # panics
///
/// this function will panic if the `dst` buffer is not exactly twice the size
/// of the `src` buffer.
///
#[inline]
pub fn into(src: &[u8], dst: &mut [u8]) {
    assert_eq!(dst.len(), src.len() * 2,"hex buff must be 2x the size of byte buff");
    for (src_byte,dst_chunk) in src.iter().zip(dst.chunks_mut(2)) {
        debug_assert!(dst_chunk.len() == 2,"chunks should always be of length 2");
        let (a,b) = byte::into(*src_byte);
        dst_chunk[0] = a;
        dst_chunk[1] = b;
    }
}


/// convert from byte values to uppercase hexadecimal.
///
/// process a slice of byte values, writing their corresponding uppercase 
/// hexadecimal representation to the supplied buffer.
///
/// # panics
///
/// this function will panic if the `dst` buffer is not exactly twice the size
/// of the `src` buffer.
///
#[inline]
pub fn into_upper(src: &[u8], dst: &mut [u8]) {
    assert_eq!(dst.len(), src.len() * 2,"hex buff must be 2x the size of byte buff");
    for (src_byte,dst_chunk) in src.iter().zip(dst.chunks_mut(2)) {
        debug_assert!(dst_chunk.len() == 2,"chunks should always be of length 2");
        let (a,b) = byte::into_upper(*src_byte);
        dst_chunk[0] = a;
        dst_chunk[1] = b;
    }
}


#[cfg(test)]
mod tests {
    use hex;

    const LOWER: &'static [u8] = b"0123456789abcdef";
    
    const UPPER: &'static [u8] = b"0123456789ABCDEF";
    
    const BYTES: &'static [u8] = &[
        0x01,0x23,0x45,0x67,0x89,0xab,0xcd,0xef
    ];


    #[test]
    fn lower() {
        let mut lower_bytes = vec![0u8;BYTES.len()];
        hex::from(LOWER,&mut lower_bytes).unwrap();
        assert_eq!(BYTES,&*lower_bytes);
        let mut lower_hex = vec![0u8;LOWER.len()];
        hex::into(BYTES,&mut lower_hex);
        assert_eq!(LOWER,&*lower_hex);
    }


    #[test]
    fn upper() {
        let mut upper_bytes = vec![0u8;BYTES.len()];
        hex::from(UPPER,&mut upper_bytes).unwrap();
        assert_eq!(BYTES,&*upper_bytes);
        let mut upper_hex = vec![0u8;UPPER.len()];
        hex::into_upper(BYTES,&mut upper_hex);
        assert_eq!(UPPER,&*upper_hex);
    }
}

