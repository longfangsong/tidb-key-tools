pub mod big {
    use byteorder::{BigEndian, ByteOrder};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_name = "big_endian_decode_u64")]
    pub fn decode_u64(code: &[u8]) -> u64 {
        BigEndian::read_u64(code)
    }

    #[wasm_bindgen(js_name = "big_endian_encode_u64")]
    pub fn encode_u64(buf: &mut [u8], n: u64) {
        BigEndian::write_u64(buf, n)
    }
}

pub mod little {
    use byteorder::{ByteOrder, LittleEndian};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_name = "little_endian_decode_u64")]
    pub fn decode_u64(code: &[u8]) -> u64 {
        LittleEndian::read_u64(code)
    }

    #[wasm_bindgen(js_name = "little_endian_encode_u64")]
    pub fn encode_u64(buf: &mut [u8], n: u64) {
        LittleEndian::write_u64(buf, n)
    }
}
