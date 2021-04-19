pub type CfName = &'static str;

pub type Value = Vec<u8>;

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

pub enum Modify {
    Delete(CfName, Key),
    Put(CfName, Key, Value),
}

pub struct TimeStamp(u64);

pub enum WriteType {
    Put,
    Delete,
    Lock,
    Rollback,
}

pub struct Write {
    pub write_type: WriteType,
    pub start_ts: TimeStamp,
    pub short_value: Option<Value>,
    pub has_overlapped_rollback: bool,
    pub gc_fence: Option<TimeStamp>,
}
