//max of 60 bytes long
struct TCP_Header {
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

impl TCP_Header {
    fn new(
        source_port_number: u16,
        destination_port_number: u16,
        sequence_number: u32,
        acknowledgement_number: u32,
        data_offset: u8,// DO is 4 bits but we have 4 more reservered right after
        flags: u8, // 6bits 
        window_size: u16,
        checksum: u16,
        urgent_pointer: u16,
        option: Vec<u8>
    ) -> Result<Self, &'static str> {
        if option.len() > 20 {
            return Err("Option is too long");
        }

        Ok(TCP_Header {
            source_port_number: source_port_number,
            destination_port_number: destination_port_number,
            sequence_number: sequence_number,
            acknowledgement_number: acknowledgement_number,
            data_offset: data_offset,
            flags: flags,
            window_size: window_size,
            checksum: checksum,
            urgent_pointer: urgent_pointer,
            option: option,
        })
    }

    fn is_oversized(&self) {
        if self.option.len() > 20 {
            return true;
        }
        else {
            return false;
        }
    }
}

