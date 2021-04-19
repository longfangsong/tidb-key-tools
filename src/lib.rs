mod mvcc;
mod utils;

use wasm_bindgen::prelude::*;

const ENC_GROUP_SIZE: usize = 8;
const ENC_MARKER: u8 = b'\xff';

fn encode_bytes(key: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    for chunk in key.chunks(8) {
        let mut fill_count = 0;
        for i in 0..ENC_GROUP_SIZE {
            result.push(if i < chunk.len() {
                chunk[i]
            } else {
                fill_count += 1;
                0
            })
        }
        result.push(ENC_MARKER - fill_count);
    }
    if result.len() == 0 || *result.last().unwrap() == 255 {
        result.extend(core::iter::repeat(0).take(ENC_GROUP_SIZE));
        result.push(ENC_MARKER - ENC_GROUP_SIZE as u8);
    }
    debug_assert_eq!(result.len() % 9, 0);
    result
}

fn encode_u64(value: u64) -> [u8; 8] {
    value.to_be_bytes()
}

fn parse_golang_fmt_print(mut code: &str) -> anyhow::Result<Vec<u8>> {
    if code.starts_with('[') {
        code = &code[1..];
    }
    if code.ends_with(']') {
        code = &code[..code.len() - 1];
    }
    let str_forms = code.split(' ');
    let mut result = Vec::with_capacity(str_forms.clone().count());
    for str_form in str_forms {
        let byte_form = u8::from_str_radix(str_form, 10)?;
        result.push(byte_form);
    }
    Ok(result)
}

fn could_be_encoded_bytes(code: &[u8]) -> bool {
    code.chunks(9)
        .all(|chunk| chunk.len() == 9 && chunk.last().unwrap().clone() > 0xff - 8)
}

fn decode_chunk(chunk: &[u8]) -> &[u8] {
    assert_eq!(chunk.len(), 9);
    let length = 8 - (0xff - chunk.last().unwrap().clone());
    &chunk[..length as usize]
}

fn decode_bytes(code: &[u8]) -> Vec<u8> {
    code.chunks(9)
        .map(decode_chunk)
        .flatten()
        .cloned()
        .collect()
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, tidb-key-guess!");
}

mod test {
    use super::*;

    #[test]
    fn test_encode_bytes() {
        let cases: Vec<(Vec<u8>, Vec<u8>)> = vec![
            (vec![], vec![0, 0, 0, 0, 0, 0, 0, 0, 247]),
            (vec![1, 2, 3], vec![1, 2, 3, 0, 0, 0, 0, 0, 250]),
            (vec![1, 2, 3, 0], vec![1, 2, 3, 0, 0, 0, 0, 0, 251]),
            (
                vec![1, 2, 3, 4, 5, 6, 7, 8],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 255, 0, 0, 0, 0, 0, 0, 0, 0, 247],
            ),
            (
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 255, 9, 0, 0, 0, 0, 0, 0, 0, 248],
            ),
        ];
        for (input, expected) in cases.into_iter() {
            let output = encode_bytes(&input);
            assert_eq!(output, expected);
        }
    }

    #[test]
    fn test_parse_golang_fmt_print() {
        let cases = vec![
            ("68 66 58 49", "DB:1"),
            ("[84 97 98 108 101 58 53 51]", "Table:53"),
        ];
        for (code, expected) in cases.into_iter() {
            let result = parse_golang_fmt_print(code).unwrap();
            let str_form = String::from_utf8(result).unwrap();
            assert_eq!(str_form, expected);
        }
    }

    #[test]
    fn test_decode_bytes() {
        let cases: Vec<(Vec<u8>, Vec<u8>)> = vec![
            (vec![], vec![0, 0, 0, 0, 0, 0, 0, 0, 247]),
            (vec![1, 2, 3], vec![1, 2, 3, 0, 0, 0, 0, 0, 250]),
            (vec![1, 2, 3, 0], vec![1, 2, 3, 0, 0, 0, 0, 0, 251]),
            (
                vec![1, 2, 3, 4, 5, 6, 7, 8],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 255, 0, 0, 0, 0, 0, 0, 0, 0, 247],
            ),
            (
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 255, 9, 0, 0, 0, 0, 0, 0, 0, 248],
            ),
        ];
        for (expected, input) in cases.into_iter() {
            let output = decode_bytes(&input);
            assert_eq!(output, expected);
        }
    }
}
