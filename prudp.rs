use crate::{counter::Counter, packet::Packet};
use std::net::UdpSocket;
use arc4::Arc4;

pub struct PRUDPClient<'a> {
    server: &'a mut PRUDPServer,
    cipher: Arc4<'a>,
    decipher: Arc4<'a>,
    signature_key: Vec<u8>,
    signature_base: u32,
    secure_key: Vec<u8>,
    server_connection_signature: Vec<u8>,
    client_connection_signature: Vec<u8>,
    session_id: u32,
    session_key: Vec<u8>,
    pid: u32,
    local_station_url: String,
    connection_id: u32,
    connected: bool,
    sequence_id_in: Counter,
    sequence_id_out: Counter,
}

impl<'a> PRUDPClient<'a> {
    pub fn new(server: &'a mut PRUDPServer) -> Self {
        Self {
            server,
            cipher: Arc4::with_key(&[0]),
            decipher: Arc4::with_key(&[0]),
            signature_key: vec![],
            signature_base: 0,
            secure_key: vec![],
            server_connection_signature: vec![],
            client_connection_signature: vec![],
            session_id: 0,
            session_key: vec![],
            pid: 0,
            local_station_url: "".to_string(),
            connection_id: 0,
            connected: false,
            sequence_id_in: Counter::default(),
            sequence_id_out: Counter::default(),
        }
    }

    pub fn get_server(&self) -> &PRUDPServer {
        self.server
    }

    pub fn set_nex_version(&mut self, nex_version: u32) {
        self.server.set_nex_version(nex_version);
    }

    pub fn get_cipher(&mut self) -> &mut Arc4<'a> {
        &mut self.cipher
    }

    pub fn get_decipher(&mut self) -> &mut Arc4<'a> {
        &mut self.decipher
    }

    pub fn get_signature_key(&self) -> &[u8] {
        &self.signature_key
    }

    pub fn get_signature_base(&self) -> u32 {
        self.signature_base
    }

    pub fn get_session_key(&self) -> &[u8] {
        &self.session_key
    }

    fn reset(&mut self) {
        unimplemented!();
    }

    fn get_address(&self) -> String {
        unimplemented!();
    }

    fn update_rc4_key(&mut self, rc4_key: Vec<u8>) {
        unimplemented!();
    }

    fn update_access_key(&mut self, access_key: String) {
        unimplemented!();
    }

    fn increase_ping_timeout_time(&mut self, seconds: u32) {
        unimplemented!();
    }

    fn start_timeout_timer(&mut self) {
        unimplemented!();
    }
}

pub struct PRUDPServer {
    socket: Option<UdpSocket>,
    access_key: String,
    prudp_version: u32,
    nex_version: u32,
    fragment_size: u16,
    use_packet_compression: bool,
    ping_timeout: u32,
    signature_version: u32,
    flags_version: u32,
    checksum_version: u32,
    kerberos_key_size: u32,
    kerberos_key_derivation: u32,
    server_version: u32,
    connection_id_counter: Counter,
}

impl Default for PRUDPServer {
    fn default() -> Self {
        Self {
            socket: None,
            access_key: "".to_string(),
            nex_version: 0,
            server_version: 0,
            use_packet_compression: false,
            prudp_version: 1,
            fragment_size: 1300,
            ping_timeout: 5,
            signature_version: 0,
            flags_version: 1,
            checksum_version: 1,
            kerberos_key_size: 32,
            kerberos_key_derivation: 0,
            connection_id_counter: Counter::default(),
        }
    }
}

impl PRUDPServer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_nex_version(&mut self, nex_version: u32) {
        self.nex_version = nex_version;
    }

    pub fn get_checksum_version(&self) -> u32 {
        self.checksum_version
    }

    pub fn get_flags_version(&self) -> u32 {
        self.flags_version
    }

    async fn listen(&mut self, addr: &str) -> Result<(), &'static str> {
        let socket = UdpSocket::bind(addr).map_err(|_| "Couldn't bind to address")?;
        self.socket = Some(socket);

        loop {
            self.handle_socket_message().await?;
        }
    }

    async fn handle_socket_message(&mut self) -> Result<(), &'static str> {
        let mut buf: Vec<u8> = vec![];
        let socket = match &self.socket {
            Some(socket) => Ok(socket),
            None => Err("No socket"),
        }?;

        let (receive_size, peer) = socket
            .recv_from(&mut buf)
            .map_err(|_| "UDP Receive error")?;

        Ok(())
    }

    fn client_connected(&mut self, client: &mut PRUDPClient) -> bool {
        unimplemented!()
    }

    fn kick(&mut self, client: &mut PRUDPClient) {
        unimplemented!()
    }

    fn send_ping(&mut self, client: &mut PRUDPClient) {
        unimplemented!()
    }

    fn acknowledge_packet<'a>(&mut self, packet: Packet, payload: Vec<u8>) {
        unimplemented!()
    }

    fn use_packet_compression(&mut self, use_packet_compression: bool) {
        unimplemented!()
    }

    fn find_client_from_pid<'a>(&self, pid: u32) -> &'a mut PRUDPClient<'a> {
        unimplemented!()
    }

    fn send<'a>(&mut self, packet: Packet) {
        unimplemented!()
    }

    fn send_fragment<'a>(&mut self, packet: Packet, fragment_id: u32) {
        unimplemented!()
    }

    fn send_raw(&mut self, conn: String, data: Vec<u8>) {
        unimplemented!()
    }
}