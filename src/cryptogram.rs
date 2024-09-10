pub trait Cryptogram {
    fn encode(&self, data: &str) -> String;
    fn decode(&self, data: &str) -> String;
}

pub struct XOR {
    pub key: String,
}

impl Cryptogram for XOR {
    fn encode(&self, data: &str) -> String {
        let data_bytes = data.as_bytes();
        let key_bytes = self.key.as_bytes();
        let key_len = key_bytes.len();
        let encoded_bytes = data_bytes
            .iter()
            .enumerate()
            .map(|(i, &byte)| {
                byte ^ key_bytes[i % key_len]
            })
            .collect();
        String::from_utf8(encoded_bytes).expect("XOR encoding failed")
    }

    fn decode(&self, data: &str) -> String {
        self.encode(data)
    }
}