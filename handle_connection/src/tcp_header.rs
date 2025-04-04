#[derive(PartialEq)] //for checking if none

//max of 60 bytes long
pub struct TCPHeader {
    // 20 bytes
    pub source_port: u16,
    pub dest_port: u16,
    pub sequence_no: u32,
    pub ack_flag: u8,
    pub syn_flag: u8,
    pub fin_flag: u8,
    pub payload_size: u16
}

impl TCPHeader {
    pub fn new(
        source_port: u16,
        dest_port: u16,
        sequence_no: u32,
        ack_flag: u8,
        syn_flag: u8,
        fin_flag: u8,
        payload_size: u16
    ) -> Self {
        TCPHeader {
            source_port,
            dest_port,
            sequence_no,
            ack_flag,
            syn_flag,
            fin_flag,
            payload_size
        }
    }
    
    //to encode the tcp header into a sequence of bytes (Big Endian)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(13); //getting our buffer and setting its max size

        buffer.extend_from_slice(&self.source_port.to_be_bytes());
        buffer.extend_from_slice(&self.dest_port.to_be_bytes());
        buffer.extend_from_slice(&self.sequence_no.to_be_bytes());
        buffer.extend_from_slice(&self.ack_flag.to_be_bytes());
        buffer.extend_from_slice(&self.syn_flag.to_be_bytes());
        buffer.extend_from_slice(&self.fin_flag.to_be_bytes());
        buffer.extend_from_slice(&self.payload_size.to_be_bytes());

        buffer
    }
}

//turns a byte stream to a tcp header
pub fn to_tcp_header(buffer: Vec<u8>) -> TCPHeader {
    let source_port = u16::from_be_bytes([
        buffer[0], buffer[1]
    ]);
    let dest_port = u16::from_be_bytes([
        buffer[2], buffer[3]
    ]);
    let sequence_no = u32::from_be_bytes([
        buffer[4], buffer[5], buffer[6], buffer[7]
    ]);
    let ack_flag = u8::from_be_bytes([
        buffer[8]
    ]);
    let syn_flag = u8::from_be_bytes([
        buffer[9]
    ]);
    let fin_flag = u8::from_be_bytes([
        buffer[10]
    ]);
    let payload_size = u16::from_be_bytes([
        buffer[11], buffer[12]
    ]);


    TCPHeader::new(
        source_port,
        dest_port,
        sequence_no,
        ack_flag,
        syn_flag,
        fin_flag,
        payload_size
    )
}

