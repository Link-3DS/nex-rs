pub struct RMCResponse {
    protocol_id: u8,
    success: u8,
    call_id: u32,
    method_id: u32,
    data: Option<Vec<u8>>,
    error_code: u32,
}

// Enums for success and error states
const ERROR_MASK: u32 = 1 << 31;

impl RMCResponse {
    // SetSuccess sets the RMCResponse payload to an instance of RMCSuccess
    pub fn set_success(&mut self, method_id: u32, data: Vec<u8>) {
        self.success = 1;
        self.method_id = method_id;
        self.data = Some(data);
    }

    // SetError sets the RMCResponse payload to an instance of RMCError
    pub fn set_error(&mut self, mut error_code: u32) {
        if error_code & ERROR_MASK == 0 {
            error_code |= ERROR_MASK;
        }
        self.success = 0;
        self.error_code = error_code;
    }

    // Converts the RMCResponse struct into a usable byte array
    pub fn bytes(&self) -> Vec<u8> {
        let mut body = Vec::new();
        body.push(self.protocol_id);
        body.push(self.success);

        if self.success == 1 {
            body.extend_from_slice(&self.call_id.to_le_bytes());
            body.extend_from_slice(&self.method_id.to_le_bytes());

            if let Some(data) = &self.data {
                body.extend_from_slice(data);
            }
        } else {
            body.extend_from_slice(&self.error_code.to_le_bytes());
            body.extend_from_slice(&self.call_id.to_le_bytes());
        }

        body
    }
}

// NewRMCResponse returns a new RMCResponse
pub fn new_rmc_response(protocol_id: u8, call_id: u32) -> RMCResponse {
    RMCResponse {
        protocol_id,
        call_id,
        success: 0,
        method_id: 0,
        data: None,
        error_code: 0,
    }
}