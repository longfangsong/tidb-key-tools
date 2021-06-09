const ENC_GROUP_SIZE: usize = 8;
const ENC_MARKER: u8 = b'\xff';

pub fn encode_bytes(key: &[u8]) -> Vec<u8> {
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
    if result.is_empty() || *result.last().unwrap() == 255 {
        result.extend(core::iter::repeat(0).take(ENC_GROUP_SIZE));
        result.push(ENC_MARKER - ENC_GROUP_SIZE as u8);
    }
    debug_assert_eq!(result.len() % 9, 0);
    result
}

fn decode_chunk(chunk: &[u8]) -> &[u8] {
    debug_assert_eq!(chunk.len(), 9);
    let length = 8 - (0xff - *chunk.last().unwrap());
    &chunk[..length as usize]
}

pub fn decode_bytes(code: &[u8]) -> Vec<u8> {
    code.chunks(9)
        .map(decode_chunk)
        .flatten()
        .cloned()
        .collect()
}

fn could_be_encoded_bytes(code: &[u8]) -> bool {
    code.chunks(9)
        .all(|chunk| chunk.len() == 9 && *chunk.last().unwrap() > 0xff - 8)
}

#[cfg(test)]
mod tests {
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
