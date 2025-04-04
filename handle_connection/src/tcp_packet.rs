pub struct TCPPacket {
    header: TCPHeader, //header
    payload: Vec<u8> //byte stream
}

impl TCPPacket {
    pub fn new(header: TCPHeader, payload: Vec<u8>) -> Self {
        TCPPacket {
            header,
            payload
        }
    }
}

pub fn calculate_checksum(buffer: &[u8]) -> u16 {
    let mut sum = 0u32;

    // Process the buffer in 16-bit chunks
    for chunk in buffer.chunks(2) {
        let value = if chunk.len() == 2 {
            u16::from_be_bytes([chunk[0], chunk[1]]) as u32
        } else {
            (chunk[0] as u32) << 8
        };
        sum = sum.wrapping_add(value);
    }

    // Fold the sum down to 16 bits and take the one's complement
    let checksum = sum + (sum >> 16);
    !(checksum as u16)
}