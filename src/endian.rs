pub mod big {
    use byteorder::{BigEndian, ByteOrder};

    pub fn decode_u64(code: &[u8]) -> u64 {
        BigEndian::read_u64(code)
    }

    pub fn encode_u64(buf: &mut [u8], n: u64) {
        BigEndian::write_u64(buf, n)
    }
}
