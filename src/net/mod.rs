use std::{mem::size_of, slice::from_raw_parts};

use self::packet::{sPCStyle, FFPacket};

pub mod crypto;
pub mod ffclient;
pub mod ffserver;
pub mod packet;

#[allow(non_snake_case)]
pub struct LoginData {
    pub iPC_UID: i64,
    pub uiFEKey: u64,
    pub uiSvrTime: u64,
    pub PCStyle: sPCStyle,
}

unsafe fn bytes_to_struct<T: FFPacket>(bytes: &[u8]) -> &T {
    // haters will call this "undefined behavior"
    let struct_ptr: *const T = bytes.as_ptr().cast();
    &*struct_ptr
}

unsafe fn struct_to_bytes<T: FFPacket>(pkt: &T) -> &[u8] {
    let sz: usize = size_of::<T>();
    let struct_ptr: *const T = pkt;
    let buf_ptr: *const u8 = struct_ptr.cast();
    from_raw_parts(buf_ptr, sz)
}
