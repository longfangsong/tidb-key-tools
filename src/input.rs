use wasm_bindgen::JsValue;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_golang_fmt_print(mut code: &str) -> Result<Vec<u8>, JsValue> {
    if code.starts_with('[') {
        code = &code[1..];
    }
    if code.ends_with(']') {
        code = &code[..code.len() - 1];
    }
    let str_forms = code.split(' ');
    let mut result = Vec::with_capacity(str_forms.clone().count());
    for str_form in str_forms {
        let byte_form = str_form
            .parse::<u8>()
            .map_err(|_| JsValue::from("Parse golang fmt.print failed"))?;
        result.push(byte_form);
    }
    Ok(result)
}

#[wasm_bindgen]
pub fn parse_hex_encoded(code: &str) -> Result<Vec<u8>, JsValue> {
    hex::decode(code).map_err(|_| JsValue::from("Invalid hex encoded"))
}

#[wasm_bindgen]
pub fn parse_rust_print(mut code: &str) -> Result<Vec<u8>, JsValue> {
    if code.starts_with('[') {
        code = &code[1..];
    }
    if code.ends_with(']') {
        code = &code[..code.len() - 1];
    }
    let str_forms = code.split(',').map(|it| it.trim());
    let mut result = Vec::with_capacity(str_forms.clone().count());
    for str_form in str_forms {
        let byte_form = str_form
            .parse::<u8>()
            .map_err(|_| JsValue::from("Invalid rust print encoded"))?;
        result.push(byte_form);
    }
    Ok(result)
}

#[wasm_bindgen]
pub fn parse_input(code: &str) -> Result<Vec<u8>, JsValue> {
    if let Ok(result) = parse_rust_print(code) {
        Ok(result)
    } else if let Ok(result) = parse_golang_fmt_print(code) {
        Ok(result)
    } else if let Ok(result) = parse_hex_encoded(code) {
        Ok(result)
    } else {
        Err(JsValue::from_str("Cannot parse input"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::db_to_kv::parse_record;

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
    fn test_decode_hex() {
        let expected = vec![
            116, 128, 0, 0, 0, 0, 0, 0, 53, 95, 114, 128, 0, 0, 0, 0, 0, 0, 1,
        ];
        assert_eq!(
            parse_hex_encoded("7480000000000000355f728000000000000001").unwrap(),
            expected
        )
    }
}
