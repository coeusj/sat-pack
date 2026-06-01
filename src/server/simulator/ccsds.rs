pub struct Packet {
    pub version: u8,
    pub r#type: u8,
    pub secondary_header: u8,
    pub apid: u16,
    pub sequence_flags: u16,
    pub sequence_count: u16,
    pub payload_length: u16,
    pub altitude: f64,
    pub velocity: f64
}

impl Packet {
    pub fn print(&self) {
        println!("[\n Version: {}\n Type: {}\n Secondary Header: {}\n APID: {}\n Sequence Flags: {}\n Sequence Count: {}\n Length: {}\n Altitude: {}\n Velocity: {}\n]",
            self.version,
            self.r#type,
            self.secondary_header,
            self.apid,
            self.sequence_flags,
            self.sequence_count,
            self.payload_length,
            self.altitude,
            self.velocity
        );
    }

    pub fn to_bin_vec(&self) -> Vec<u16> {
        let mut res: Vec<u16> = Vec::new();

        let packet_id = ((self.version as u16) << 13) | ((self.r#type as u16) << 12) | ((self.secondary_header as u16) << 11) | self.apid;
        res.push(packet_id);

        let packet_sequence: u16 = (self.sequence_flags << 14) | self.sequence_count;
        res.push(packet_sequence);

        res.push(self.payload_length);

        let altitude_bytes = self.altitude.to_be_bytes();
        for i in (0..altitude_bytes.len()).step_by(2) {
            let first = (altitude_bytes[i] as u16) << 8;
            let second = altitude_bytes[i+1] as u16;
            let chunk = first | second;
            res.push(chunk);
        }

        let velocity_bytes = self.velocity.to_be_bytes();
        for i in (0..velocity_bytes.len()).step_by(2) {
            let first = (velocity_bytes[i] as u16) << 8;
            let second = velocity_bytes[i+1] as u16;
            let chunk = first | second;
            res.push(chunk);
        }

        return res;
    }
}
