use crate::client::PRUDPClient;
use crate::rmc::rmc_request::RMCRequest;

pub struct Packet<'a> {
    sender: Option<&'a PRUDPClient>,
    data: Vec<u8>,
    version: u8,
    source: u8,
    destination: u8,
    packet_type: u16,
    flags: u16,
    session_id: u8,
    signature: Vec<u8>,
    sequence_id: u16,
    connection_signature: Vec<u8>,
    fragment_id: u8,
    payload: Vec<u8>,
    rmc_request: RMCRequest,
}

impl<'a> Packet<'a> {
    pub fn new(client: Option<&'a PRUDPClient>, data: Vec<u8>) -> Self {
        Packet {
            sender: client,
            data,
            version: 0,
            source: 0,
            destination: 0,
            packet_type: 0,
            flags: 0,
            session_id: 0,
            signature: vec![],
            sequence_id: 0,
            connection_signature: vec![],
            fragment_id: 0,
            payload: vec![],
            rmc_request: RMCRequest::default(),
        }
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn sender(&self) -> Option<&PRUDPClient> {
        self.sender
    }

    pub fn set_version(&mut self, version: u8) {
        self.version = version;
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn set_source(&mut self, source: u8) {
        self.source = source;
    }

    pub fn source(&self) -> u8 {
        self.source
    }

    pub fn set_destination(&mut self, destination: u8) {
        self.destination = destination;
    }

    pub fn destination(&self) -> u8 {
        self.destination
    }

    pub fn set_type(&mut self, packet_type: u16) {
        self.packet_type = packet_type;
    }

    pub fn packet_type(&self) -> u16 {
        self.packet_type
    }

    pub fn set_flags(&mut self, bitmask: u16) {
        self.flags = bitmask;
    }

    pub fn flags(&self) -> u16 {
        self.flags
    }

    pub fn has_flag(&self, flag: u16) -> bool {
        self.flags & flag != 0
    }

    pub fn add_flag(&mut self, flag: u16) {
        self.flags |= flag;
    }

    pub fn clear_flag(&mut self, flag: u16) {
        self.flags &= !flag;
    }

    pub fn set_session_id(&mut self, session_id: u8) {
        self.session_id = session_id;
    }

    pub fn session_id(&self) -> u8 {
        self.session_id
    }

    pub fn set_signature(&mut self, signature: Vec<u8>) {
        self.signature = signature;
    }

    pub fn signature(&self) -> &Vec<u8> {
        &self.signature
    }

    pub fn set_sequence_id(&mut self, sequence_id: u16) {
        self.sequence_id = sequence_id;
    }

    pub fn sequence_id(&self) -> u16 {
        self.sequence_id
    }

    pub fn set_connection_signature(&mut self, connection_signature: Vec<u8>) {
        self.connection_signature = connection_signature;
    }

    pub fn connection_signature(&self) -> &Vec<u8> {
        &self.connection_signature
    }

    pub fn set_fragment_id(&mut self, fragment_id: u8) {
        self.fragment_id = fragment_id;
    }

    pub fn fragment_id(&self) -> u8 {
        self.fragment_id
    }

    pub fn set_payload(&mut self, payload: Vec<u8>) {
        self.payload = payload;
    }

    pub fn payload(&self) -> &Vec<u8> {
        &self.payload
    }

    pub fn rmc_request(&self) -> &RMCRequest {
        &self.rmc_request
    }
}