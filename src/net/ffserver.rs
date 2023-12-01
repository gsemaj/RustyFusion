use polling::{Event, PollMode, Poller};

use std::{
    collections::HashMap,
    io::{ErrorKind, Result},
    net::{SocketAddr, TcpListener, TcpStream},
    time::Duration,
};

use crate::{
    error::{log, Severity},
    state::ServerState,
};

use super::{
    ffclient::{ClientType, FFClient},
    DisconnectCallback, PacketCallback,
};

const EPOLL_KEY_SELF: usize = 0;

pub struct FFServer {
    poll_timeout: Option<Duration>,
    sock: TcpListener,
    poller: Poller,
    next_epoll_key: usize,
    clients: HashMap<usize, FFClient>,
}

impl FFServer {
    pub fn new(addr: &str, poll_timeout: Option<Duration>) -> Result<Self> {
        let server: Self = Self {
            poll_timeout,
            sock: TcpListener::bind(addr)?,
            poller: Poller::new()?,
            next_epoll_key: EPOLL_KEY_SELF + 1,
            clients: HashMap::new(),
        };
        server.sock.set_nonblocking(true)?;
        server
            .poller
            .add_with_mode(&server.sock, Event::all(EPOLL_KEY_SELF), PollMode::Level)?;
        Ok(server)
    }

    pub fn connect(&mut self, addr: &str, cltype: ClientType) -> Option<&mut FFClient> {
        let addr: SocketAddr = addr.parse().expect("Bad address");
        let stream = TcpStream::connect(addr);
        if let Ok(stream) = stream {
            let conn_data: (TcpStream, SocketAddr) = (stream, addr);
            let key: usize = self.register_client(conn_data).unwrap();
            let client: &mut FFClient = self.clients.get_mut(&key).unwrap();
            client.set_client_type(cltype);
            return Some(client);
        }
        None
    }

    pub fn poll(
        &mut self,
        pkt_handler: PacketCallback,
        dc_handler: Option<DisconnectCallback>,
        state: &mut ServerState,
    ) -> Result<()> {
        let mut events: Vec<Event> = Vec::new();
        if let Err(e) = self.poller.wait(&mut events, self.poll_timeout) {
            match e.kind() {
                ErrorKind::Interrupted => return Ok(()), // this is fine
                _ => {
                    return Err(e);
                }
            }
        }

        let mut dc_handler = dc_handler;
        for ev in events.iter() {
            if ev.key == EPOLL_KEY_SELF {
                let conn_data: (TcpStream, SocketAddr) = self.sock.accept()?;
                log(
                    Severity::Debug,
                    &format!("New connection from {}", conn_data.1),
                );
                self.register_client(conn_data)?;
            } else {
                if !ev.readable || !ev.writable {
                    continue;
                };
                let clients: &mut HashMap<usize, FFClient> = &mut self.clients;
                let client: &mut FFClient = clients.get_mut(&ev.key).unwrap();
                let addr = client.get_addr();
                let res = client
                    .read_packet()
                    .and_then(|pkt_id| pkt_handler(ev.key, clients, pkt_id, state));

                if let Err(e) = res {
                    log(e.get_severity(), &format!("{} ({})", e.get_msg(), addr));
                    if e.should_dc_client() {
                        if let Some(callback) = dc_handler.as_mut() {
                            callback(ev.key, clients, state);
                        };
                        self.unregister_client(ev.key)?.unwrap();
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get_endpoint(&self) -> String {
        self.sock.local_addr().unwrap().to_string()
    }

    fn get_next_epoll_key(&mut self) -> usize {
        let key: usize = self.next_epoll_key;
        self.next_epoll_key += 1;
        key
    }

    fn register_client(&mut self, conn_data: (TcpStream, SocketAddr)) -> Result<usize> {
        let key: usize = self.get_next_epoll_key();
        self.poller
            .add_with_mode(&conn_data.0, Event::all(key), PollMode::Level)?;
        self.clients.insert(key, FFClient::new(conn_data));
        Ok(key)
    }

    fn unregister_client(&mut self, key: usize) -> Result<Option<FFClient>> {
        let client: &FFClient = self.clients.get(&key).unwrap();
        self.poller.delete(client.get_sock())?;
        Ok(self.clients.remove(&key))
    }
}
