use std::convert::TryInto;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Record {
    // IMO these fields should be unsigned
    // But, TiDB is TiDB 🤷‍
    pub table_id: i64,
    pub row_id: i64,
}

const SIGN_MASK: u64 = 0x8000000000000000;

#[wasm_bindgen]
pub fn parse_record(code: &[u8]) -> Result<Record, JsValue> {
    if code[0] != b't' || code[9] != b'_' || code[10] != b'r' {
        Err(JsValue::from("Invalid record bytes"))
    } else {
        let table_id_bytes: [u8; 8] = code[1..=8]
            .try_into()
            .map_err(|_| JsValue::from("Invalid record bytes"))?;
        // who invent this evil encoding method?
        let table_id = (u64::from_be_bytes(table_id_bytes) ^ SIGN_MASK) as i64;
        let row_id_bytes: [u8; 8] = code[11..]
            .try_into()
            .map_err(|_| JsValue::from("Invalid record bytes"))?;
        let row_id = (u64::from_be_bytes(row_id_bytes) ^ SIGN_MASK) as i64;
        Ok(Record { table_id, row_id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_record() {
        let code = vec![
            116, 128, 0, 0, 0, 0, 0, 0, 53, 95, 114, 128, 0, 0, 0, 0, 0, 0, 1,
        ];
        let result = parse_record(&code).unwrap();
        assert_eq!(result.table_id, 53);
        assert_eq!(result.row_id, 1);
    }
}
