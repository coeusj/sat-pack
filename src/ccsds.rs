#[derive(Debug)]
pub struct CCSDS {
    version: u8,
    r#type: u8,
    secondary_header: u8,
    apid: u16,
    sequence_flags: u8,
    sequence_count: u16,
    payload_len: u16,
    altitude: f64,
    velocity: f64
}

impl CCSDS {
    pub fn print(&self) {
        println!("[\n Version: {}\n Type: {}\n Secondary Header: {}\n APID: {}\n Sequence Flags: {}\n Sequence Count: {}\n Length: {}\n Altitude: {}\n Velocity: {}\n]",
            self.version,
            self.r#type,
            self.secondary_header,
            self.sequence_flags,
            self.sequence_count,
            self.apid,
            self.payload_len,
            self.altitude,
            self.velocity
        );
    }
}

pub fn generate() -> Vec<u16> {
    let mut res: Vec<u16> = Vec::new();

    // Packet ID
    // 16bit = version + type + secondary header
    let version: u16 = 0;
    let packet_type: u16 = 0;
    let secondary_header: u16 = 0;
    let apid: u16 = 10;
    let packet_id = (version << 13) | (packet_type << 12) | (secondary_header << 11) | apid;
    res.push(packet_id);

    // Packet Sequence
    // 16bit = sequence flags + sequence count
    let sequence_flags: u16 = 3;
    let sequence_count: u16 = 1;
    let packet_sequence: u16 = (sequence_flags << 14) | sequence_count;
    res.push(packet_sequence);

    // Packet Length. The payload are 2 float64 = 16byte. So 16 - 1 = 15
    let payload_length: u16 = 16 - 1;
    res.push(payload_length);

    // Write Altitude payload
    let altitude: f64 = 120500.45; // in meters
    let altitude_bytes = altitude.to_be_bytes();
    for i in (0..altitude_bytes.len()).step_by(2) {
        let first = (altitude_bytes[i] as u16) << 8;
        let second = altitude_bytes[i+1] as u16;
        let chunk = first | second;
        res.push(chunk);
    }

    // Write Velocity payload
    let velocity: f64 = 1540.32; // in m/s
    let velocity_bytes = velocity.to_be_bytes();
    for i in (0..velocity_bytes.len()).step_by(2) {
        let first = (velocity_bytes[i] as u16) << 8;
        let second = velocity_bytes[i+1] as u16;
        let chunk = first | second;
        res.push(chunk);
    }

    return res;
}

pub fn read(packet: Vec<u16>) -> CCSDS {
    let packet_id = &packet[0];
    let version = ((packet_id >> 13) & 0b111) as u8;
    let r#type = ((packet_id >> 12) & 0b1) as u8;
    let secondary_header = ((packet_id >> 13) & 0b1) as u8;
    let apid = packet_id & 0b11111111111;

    let packet_sequence_control = &packet[1];
    let sequence_flags = ((packet_sequence_control >> 14) & 0b11) as u8;
    let sequence_count = packet_sequence_control & 0b11111111111111;

    let packet_length = &packet[2];

    let altitude = &packet[3..=6];
    let mut altitute_bits: u64 = 0;
    for (i, &value) in altitude.iter().enumerate() {
        altitute_bits |= (value as u64) << ((3 - i) * 16);
    }
    let altitude_val = f64::from_bits(altitute_bits);

    let velocity = &packet[7..=10];
    let vel_byte_list: Vec<u8> = velocity.iter()
        .flat_map(|&num| num.to_be_bytes())
        .take(8)
        .collect();
    let mut velocity_val: f64 = 0.0;
    if let Ok(byte_array) = vel_byte_list.try_into() {
        velocity_val = f64::from_be_bytes(byte_array);
    }

    return CCSDS {
        version:  version,
        r#type: r#type,
        secondary_header: secondary_header,
        apid: apid,
        sequence_flags: sequence_flags,
        sequence_count: sequence_count,
        payload_len: *packet_length,
        altitude: altitude_val,
        velocity: velocity_val
    }
}