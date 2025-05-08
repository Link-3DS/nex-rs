pub struct RMCRequest {
    protocol_id: u8,
    call_id: u32,
    method_id: u32,
    parameters: Vec<u8>,
}

impl RMCRequest {
    pub fn new() -> Self {
        RMCRequest {
            protocol_id: 0,
            call_id: 0,
            method_id: 0,
            parameters: Vec::new(),
        }
    }

    pub fn protocol_id(&self) -> u8 {
        self.protocol_id
    }

    pub fn call_id(&self) -> u32 {
        self.call_id
    }

    pub fn method_id(&self) -> u32 {
        self.method_id
    }

    pub fn parameters(&self) -> &Vec<u8> {
        &self.parameters
    }

    pub fn set_protocol_id(&mut self, protocol_id: u8) {
        self.protocol_id = protocol_id;
    }

    pub fn set_call_id(&mut self, call_id: u32) {
        self.call_id = call_id;
    }

    pub fn set_method_id(&mut self, method_id: u32) {
        self.method_id = method_id;
    }

    pub fn set_parameters(&mut self, parameters: Vec<u8>) {
        self.parameters = parameters;
    }
}