use simulator::ccsds::Packet;

pub mod simulator;

fn main() {
    let mut sim = simulator::Simulator {
        packet: Packet {
            version: 0,
            r#type: 0,
            secondary_header: 0,
            apid: 10,
            sequence_flags: 3,
            sequence_count: 1,
            payload_length: 15,
            altitude: 120500.45,
            velocity: 1540.32
        }
    };

    sim.start(10);
}
