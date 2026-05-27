pub mod ccsds;

fn main() {
    let ccsds_bin = ccsds::generate();
    let ccsds_packet = ccsds::read(ccsds_bin);
    ccsds_packet.print();
}
