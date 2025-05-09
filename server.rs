use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
pub struct PRUDPServer {
    socket: Option<Arc<Mutex<UdpSocket>>>,
    access_key: String,
    prudp_version: i32,
    nex_version: i32,
    fragment_size: i32,
    flags_version: i32,
    kerberos_key_size: i32
}

impl PRUDPServer {
    pub fn new() -> Self {
        Self {
            socket: None,
            access_key: String::new(),
            prudp_version: 1,
            nex_version: 0,
            fragment_size: 1300,
            flags_version: 1,
            kerberos_key_size: 32,
        }
    }

    pub fn listen(&mut self, address: &str) {
        let socket = Arc::new(Mutex::new(
            UdpSocket::bind(address).expect("Failed to bind UDP socket")
        ));
        
        self.socket = Some(socket.clone());
    
        for _ in 0..num_cpus::get() {
            let server_clone = Arc::new(Mutex::new(self.clone()));
            thread::spawn(move || {
                server_clone.lock().unwrap().listen_datagram();
            });
        }
    
        println!("Server listening on {}", address);
    }

    pub fn listen_datagram(&self) {
        let mut err: Option<String> = None;
    
        while err.is_none() {
            err = self.handle_socket_message();
        }
    
        if let Some(e) = err {
            panic!("{}", e);
        }
    }

    pub fn handle_socket_message(&self) -> Option<String> {
        None // TODO
    }

    pub fn get_socket(&self) -> UdpSocket {
        let socket = self.socket.as_ref().expect("Socket is not initialized").lock().unwrap();
        socket.try_clone().unwrap()
    }    

    pub fn set_socket(&mut self, socket: Arc<Mutex<UdpSocket>>) {
        self.socket = Some(socket);
    }    

    pub fn get_prudp_version(&self) -> &i32 {
        &self.prudp_version
    }

    pub fn set_prudp_version(&mut self, prudp_version: i32) {
        self.prudp_version = prudp_version
    }

    pub fn get_nex_version(&self) -> &i32 {
        &self.nex_version
    }

    pub fn set_nex_version(&mut self, nex_version: i32) {
        self.nex_version = nex_version
    }

    pub fn get_access_key(&self) -> &String {
        &self.access_key
    }

    pub fn set_access_key(&mut self, access_key: String) {
        self.access_key = access_key
    }

    pub fn get_kerberos_key_size(&self) -> &i32 {
        &self.kerberos_key_size
    }

    pub fn set_kerberos_key_size(&mut self, kerberos_key_size: i32) {
        self.kerberos_key_size = kerberos_key_size
    }

    pub fn set_fragment_size(&mut self, fragment_size: i32) {
        self.fragment_size = fragment_size
    }
}