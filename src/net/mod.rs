use std::{collections::HashMap, mem::size_of, slice::from_raw_parts};

use self::{
    ffclient::FFClient,
    packet::{sPCStyle, FFPacket, PacketID},
};
use crate::Result;

pub mod crypto;
pub mod ffclient;
pub mod ffserver;
pub mod packet;

pub type PacketCallback = fn(usize, &mut HashMap<usize, FFClient>, PacketID) -> Result<()>;
pub type DisconnectCallback = fn(FFClient);

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

pub struct ClientMap<'a> {
    key: usize,
    clients: &'a mut HashMap<usize, FFClient>,
}
impl<'a> ClientMap<'a> {
    pub fn new(key: usize, clients: &'a mut HashMap<usize, FFClient>) -> Self {
        Self { key, clients }
    }

    pub fn get_self(&mut self) -> &mut FFClient {
        self.clients.get_mut(&self.key).unwrap()
    }

    pub fn get_all(&mut self) -> impl Iterator<Item = &mut FFClient> {
        self.clients.values_mut()
    }

    pub fn get_all_but_self(&mut self) -> impl Iterator<Item = &mut FFClient> {
        self.clients
            .iter_mut()
            .filter_map(|(key, client)| if *key != self.key { Some(client) } else { None })
    }
}
