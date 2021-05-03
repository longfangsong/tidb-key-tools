use integer_encoding::VarInt;

pub fn decode_u64(code: &[u8]) -> (u64, usize) {
    u64::decode_var(code).unwrap()
}

pub fn encode_u64(i: u64) -> Vec<u8> {
    i.encode_var_vec()
}
