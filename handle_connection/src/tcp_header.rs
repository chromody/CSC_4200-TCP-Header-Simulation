//max of 60 bytes long
pub struct TCPHeader {
    // 20 bytes
    pub source_port_number: u16,
    pub destination_port_number: u16,
    pub sequence_number: u32,
    pub acknowledgement_number: u32,
    pub data_offset_reserved_flags: u16, // Combined field for Data Offset (4 bits), Reserved (3 bits), and Flags (9 bits)
    pub window_size: u16,
    pub checksum: u16,
    pub urgent_pointer: u16,
    // 40 bytes
    pub option: Vec<u8> // so a max vector size of 40
}

impl TCPHeader {
    pub fn new(
        source_port_number: u16,
        destination_port_number: u16,
        sequence_number: u32,
        acknowledgement_number: u32,
        flags: u16, // including reserved, 9 bits in total
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

        // Pack data_offset, reserved (3 bits as 0), and flags (9 bits) into a single u16
        let reserved_bits = 0b000; // 3 reserved bits set to 0
        let data_offset_reserved_flags = ((data_offset as u16) << 12) | ((reserved_bits as u16) << 9) | (flags & 0x1FF);

        Ok(TCPHeader {
            source_port_number,
            destination_port_number,
            sequence_number,
            acknowledgement_number,
            data_offset_reserved_flags,
            window_size,
            checksum,
            urgent_pointer,
            option,
        })
    }

    pub fn data_offset(&self) -> u8 {
        (self.data_offset_reserved_flags >> 12) as u8
    }

    pub fn flags(&self) -> u16 {
        self.data_offset_reserved_flags & 0x1FF
    }

    pub fn is_syn(&self) -> bool {
        (self.flags() & 0x02) != 0 // Check SYN bit
    }

    pub fn is_fin(&self) -> bool {
        (self.flags() & 0x01) != 0 // Check FIN bit
    }

    //checking if option is too long, may be used later
    pub fn is_oversized(&self) -> bool {
        self.option.len() > 40 
    }
    
    //to encode the tcp header into a sequence of bytes (Big Endian)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(20 + self.option.len()); //getting our buffer and setting its max size

        buffer.extend_from_slice(&self.source_port_number.to_be_bytes());
        buffer.extend_from_slice(&self.destination_port_number.to_be_bytes());
        buffer.extend_from_slice(&self.sequence_number.to_be_bytes());
        buffer.extend_from_slice(&self.acknowledgement_number.to_be_bytes());
        buffer.extend_from_slice(&self.data_offset_reserved_flags.to_be_bytes());
        buffer.extend_from_slice(&self.window_size.to_be_bytes());
        buffer.extend_from_slice(&self.checksum.to_be_bytes());
        buffer.extend_from_slice(&self.urgent_pointer.to_be_bytes());

        buffer.extend_from_slice(&self.option);

        buffer
    }
}

//turns a byte stream to a tcp header
pub fn to_tcp_header(buffer: Vec<u8>) -> Result<TCPHeader, &'static str> {
    let buffer_size = buffer.len();

    let source_port_number = u16::from_be_bytes([
        buffer[0], buffer[1]
    ]);
    let destination_port_number = u16::from_be_bytes([
        buffer[2], buffer[3]
    ]);
    let sequence_number = u32::from_be_bytes([
        buffer[4], buffer[5], buffer[6], buffer[7]
    ]);
    let acknowledgement_number = u32::from_be_bytes([
        buffer[8], buffer[9], buffer[10], buffer[11]
    ]);
    let data_offset_reserved_flags = u16::from_be_bytes([
        buffer[12], buffer[13]
    ]);
    let window_size = u16::from_be_bytes([
        buffer[14], buffer[15]
    ]);
    let checksum = u16::from_be_bytes([
        buffer[16], buffer[17]
    ]);
    let urgent_pointer = u16::from_be_bytes([
        buffer[18], buffer[19]
    ]);
    let option = buffer[20..buffer_size].to_vec();

    TCPHeader::new(source_port_number,
        destination_port_number,
        sequence_number,
        acknowledgement_number,
        data_offset_reserved_flags,
        window_size,
        checksum,
        urgent_pointer,
        option
    )
}

