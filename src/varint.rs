use integer_encoding::VarInt;

pub fn decode_u64(code: &[u8]) -> (u64, &[u8]) {
    let (result, bytes_read) = u64::decode_var(code).unwrap();
    (result, &code[bytes_read..])
}

pub fn encode_u64(i: u64) -> Vec<u8> {
    i.encode_var_vec()
}
