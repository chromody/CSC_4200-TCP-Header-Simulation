pub struct TCPPacket {
    header: TCPHeader, //header
    payload: Vec<u8> //byte stream
}

impl TCPPacket {
    pub fn new(mut header: TCPHeader, payload: Vec<u8>) -> Result<Self, &'static str> {
        header.payload_size = u16::try_from(payload.len()).expect("Value too large for u16");

        Ok(TCPPacket {
            header,
            payload
        })
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = self.header.to_bytes();
        buffer.extend_from_slice(&self.payload);

        buffer
    }
}

pub fn to_tcp_packet(buffer: Vec<u8>) -> Result<TCPPacket, &'static str> {
    let buffer_size = buffer.len();

    let header_buffer = buffer[0..13].to_vec();
    let header = to_tcp_header(header_buffer);
    let payload = buffer[13..buffer_size].to_vec();

    TCPPacket::new(header, payload)
}