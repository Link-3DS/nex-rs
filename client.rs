use std::net::{UdpSocket, SocketAddr};
use crate::server::PRUDPServer;

pub struct PRUDPClient {
    address: UdpSocket,
    server: PRUDPServer,
    signature_base: i32,
    pid: u32,
    local_station_url: String,
}

impl PRUDPClient {
    pub fn new(address: UdpSocket, server: PRUDPServer) -> Self {
        let mut client = PRUDPClient {
            address,
            server,
            signature_base: 0,
            pid: 0,
            local_station_url: String::new(),
        };

        client.reset();

        client
    }

    pub fn reset(&mut self) {
        unimplemented!(); // TODO
    }

    pub fn get_address(&self) -> &UdpSocket {
        &self.address
    }

    pub fn get_server(&self) -> &PRUDPServer {
        &self.server
    }

    pub fn get_cipher(&self) {
        unimplemented!(); // TODO
    }

    pub fn get_decipher(&self) {
        unimplemented!(); // TODO
    }

    pub fn get_signature_base(&self) -> &i32 {
        &self.signature_base
    }

    pub fn set_pid(&mut self, pid: u32) {
        self.pid = pid;
    }

    pub fn get_pid(&self) -> &u32 {
        &self.pid
    }

    pub fn set_station_url(&mut self, local_station_url: String) {
        self.local_station_url = local_station_url;
    }

    pub fn get_station_url(&self) -> &String {
        &self.local_station_url
    }
}