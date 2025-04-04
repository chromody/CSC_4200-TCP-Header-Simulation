//max of 60 bytes long
struct TCPHeader {
    // 20 bytes
    source_port_number: u16,
    destination_port_number: u16,
    sequence_number: u32,
    acknowledgement_number: u32,
    data_offset: u8,// DO is 4 bits but we have 4 more reservered right after
    flags: u8, // 6bits 
    window_size: u16,
    checksum: u16,
    urgent_pointer: u16,
    // 40 bytes
    option: Vec<u8> // so a max vector size of 40
}

impl TCPHeader {
    fn new(
        source_port_number: u16,
        destination_port_number: u16,
        sequence_number: u32,
        acknowledgement_number: u32,
        flags: u8, // 6bits 
        window_size: u16,
        checksum: u16,
        urgent_pointer: u16,
        option: Vec<u8>
    ) -> Result<Self, &'static str> {
        if option.len() > 40 {
            return Err("Option is too long");
        }

        // Compute data offset dynamically (header size / 4)
        let data_offset = (5 + (option.len() / 4)) as u8; // Min: 5 (20 bytes), Max: 15 (60 bytes)

        if data_offset > 15 {
            return Err("Invalid data offset");
        }

        Ok(TCPHeader {
            source_port_number,
            destination_port_number,
            sequence_number,
            acknowledgement_number,
            data_offset,
            flags,
            window_size,
            checksum,
            urgent_pointer,
            option,
        })
    }

    //checking if option is too long, may be used later
    fn is_oversized(&self) -> bool {
        self.option.len() > 40 
    }
    
    //to encode the tcp header into a sequence of bytes (Big Endian)
    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(20 + self.option.len()); //getting our buffer and setting its max size

        buffer.extend_from_slice(&self.source_port_number.to_be_bytes());
        buffer.extend_from_slice(&self.destination_port_number.to_be_bytes());
        buffer.extend_from_slice(&self.sequence_number.to_be_bytes());
        buffer.extend_from_slice(&self.acknowledgement_number.to_be_bytes());

        //byte manipulation since data_offset is typically only 4 bits
        let data_offset_flags = (self.data_offset << 4) | (self.flags & 0x3F);
        buffer.push(data_offset_flags);

        buffer.extend_from_slice(&self.flags.to_be_bytes());
        buffer.extend_from_slice(&self.window_size.to_be_bytes());
        buffer.extend_from_slice(&self.checksum.to_be_bytes());
        buffer.extend_from_slice(&self.urgent_pointer.to_be_bytes());

        buffer.extend_from_slice(&self.option);

        buffer
    }
}

//turns a byte stream to a tcp header
pub fn to_tcp_header(buffer: Vec<u8>) -> TCPHeader {

}

