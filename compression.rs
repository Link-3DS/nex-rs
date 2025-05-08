pub trait Compression {
    fn compress(&self, data: &[u8]) -> Vec<u8>;
    fn decompress(&self, data: &[u8]) -> Vec<u8>;
}

pub struct DummyCompression;

impl Compression for DummyCompression {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }

    fn decompress(&self, data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }
}

pub struct ZLibCompression;

impl Compression for ZLibCompression {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }

    fn decompress(&self, data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }
}