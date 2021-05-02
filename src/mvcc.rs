use crate::endian;
use crate::varint;
use wasm_bindgen::prelude::*;
pub type CfName = &'static str;

pub type Value = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Key(Vec<u8>);

const FLAG_PUT: u8 = b'P';
const FLAG_DELETE: u8 = b'D';
const FLAG_LOCK: u8 = b'L';
const FLAG_ROLLBACK: u8 = b'R';
const FLAG_OVERLAPPED_ROLLBACK: u8 = b'R';
const GC_FENCE_PREFIX: u8 = b'F';
pub const CF_DEFAULT: CfName = "default";
pub const CF_LOCK: CfName = "lock";
pub const CF_WRITE: CfName = "write";
pub const SHORT_VALUE_MAX_LEN: usize = 255;
pub const SHORT_VALUE_PREFIX: u8 = b'v';

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Modify {
    Delete(CfName, Key),
    Put(CfName, Key, Value),
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeStamp(u64);

impl From<u64> for TimeStamp {
    fn from(t: u64) -> Self {
        TimeStamp(t)
    }
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WriteType {
    Put,
    Delete,
    Lock,
    Rollback,
}

impl WriteType {
    pub fn from_u8(b: u8) -> Option<WriteType> {
        match b {
            FLAG_PUT => Some(WriteType::Put),
            FLAG_DELETE => Some(WriteType::Delete),
            FLAG_LOCK => Some(WriteType::Lock),
            FLAG_ROLLBACK => Some(WriteType::Rollback),
            _ => None,
        }
    }

    fn to_u8(self) -> u8 {
        match self {
            WriteType::Put => FLAG_PUT,
            WriteType::Delete => FLAG_DELETE,
            WriteType::Lock => FLAG_LOCK,
            WriteType::Rollback => FLAG_ROLLBACK,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Write {
    pub write_type: WriteType,
    pub start_ts: TimeStamp,
    short_value: Option<Value>,
    pub has_overlapped_rollback: bool,
    pub gc_fence: Option<TimeStamp>,
}

#[wasm_bindgen]
impl Write {
    fn parse_rust(mut b: &[u8]) -> anyhow::Result<Write> {
        let write_type_bytes = b[0];
        b = &b[1..];
        let write_type = WriteType::from_u8(write_type_bytes).unwrap();
        let (start_ts_u64, b_) = varint::decode_u64(b);
        b = b_;
        let start_ts = start_ts_u64.into();

        let mut short_value = None;
        let mut has_overlapped_rollback = false;
        let mut gc_fence = None;

        while !b.is_empty() {
            let prefix = b[0];
            b = &b[1..];
            match prefix {
                SHORT_VALUE_PREFIX => {
                    let len = b[0];
                    b = &b[1..];
                    if b.len() < len as usize {
                        panic!(
                            "content len [{}] shorter than short value len [{}]",
                            b.len(),
                            len,
                        );
                    }
                    short_value = Some(b[..len as usize].to_vec());
                    b = &b[len as usize..];
                }
                FLAG_OVERLAPPED_ROLLBACK => {
                    has_overlapped_rollback = true;
                }
                GC_FENCE_PREFIX => gc_fence = Some(endian::big::decode_u64(&b).into()),
                _ => {
                    // To support forward compatibility, all fields should be serialized in order
                    // and stop parsing if meets an unknown byte.
                    break;
                }
            }
        }

        Ok(Write {
            write_type,
            start_ts,
            short_value,
            has_overlapped_rollback,
            gc_fence,
        })
    }

    pub fn parse(b: &[u8]) -> Result<Write, JsValue> {
        Self::parse_rust(b).map_err(|_| JsValue::from_str("Cannot parse Write!"))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut b = Vec::new();
        b.push(self.write_type.clone().to_u8());
        b.extend_from_slice(&varint::encode_u64(self.start_ts.0));
        if let Some(v) = &self.short_value {
            b.push(SHORT_VALUE_PREFIX);
            b.push(v.len() as u8);
            b.extend_from_slice(v);
        }
        if self.has_overlapped_rollback {
            b.push(FLAG_OVERLAPPED_ROLLBACK);
        }
        if let Some(ts) = &self.gc_fence {
            b.push(GC_FENCE_PREFIX);
            let mut v = Vec::with_capacity(8);
            endian::big::encode_u64(&mut v, ts.0);
            b.extend_from_slice(&v);
        }
        b
    }

    #[wasm_bindgen(getter)]
    pub fn short_value(&self) -> Option<Value> {
        self.short_value.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_short_value(&mut self, value: Option<Value>) {
        self.short_value = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_write() {
        let cases = vec![
            (
                vec![80u8, 0, 118, 1, 0],
                Write {
                    write_type: WriteType::Put,
                    start_ts: TimeStamp(0),
                    short_value: Some(vec![0]),
                    has_overlapped_rollback: false,
                    gc_fence: None,
                },
            ),
            (
                vec![68, 129, 128, 144, 171, 237, 172, 172, 242, 5],
                Write {
                    write_type: WriteType::Delete,
                    start_ts: TimeStamp(424659320104550401),
                    short_value: None,
                    has_overlapped_rollback: false,
                    gc_fence: None
                }
            ),
            (
                vec![
                    80, 142, 128, 192, 164, 235, 172, 172, 242, 5, 118, 109, 123, 34, 118, 101,
                    114, 115, 105, 111, 110, 34, 58, 56, 44, 34, 116, 121, 112, 101, 34, 58, 51,
                    44, 34, 115, 99, 104, 101, 109, 97, 95, 105, 100, 34, 58, 51, 44, 34, 116, 97,
                    98, 108, 101, 95, 105, 100, 34, 58, 49, 53, 44, 34, 111, 108, 100, 95, 116, 97,
                    98, 108, 101, 95, 105, 100, 34, 58, 48, 44, 34, 111, 108, 100, 95, 115, 99,
                    104, 101, 109, 97, 95, 105, 100, 34, 58, 48, 44, 34, 97, 102, 102, 101, 99,
                    116, 101, 100, 95, 111, 112, 116, 105, 111, 110, 115, 34, 58, 110, 117, 108,
                    108, 125,
                ],
                Write {
                    write_type: WriteType::Put,
                    start_ts: TimeStamp(424659319553785870),
                    short_value: Some(b"{\"version\":8,\"type\":3,\"schema_id\":3,\"table_id\":15,\"old_table_id\":0,\"old_schema_id\":0,\"affected_options\":null}".to_vec()),
                    has_overlapped_rollback: false,
                    gc_fence: None,
                },
            ),
        ];
        for (source, expected) in cases {
            let result = Write::parse(&source).unwrap();
            assert_eq!(result, expected);
        }
    }
}
