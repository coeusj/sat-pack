use serde::Deserialize;
use config::{Config, ConfigError, File};

use ccsds::Packet;

pub mod ccsds;

#[derive(Deserialize)]
pub struct CCSDSConf {
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

#[derive(Deserialize)]
pub struct SimConf {
    pub loop_delay: u64
}

#[derive(Deserialize)]
pub struct Settings {
    pub sim_conf: SimConf,
    pub ccsds_conf: CCSDSConf
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("Settings"))
            .add_source(config::Environment::with_prefix("CCSDS_APP").separator("__"))
            .build()?;

        config.try_deserialize()
    }
}

pub struct Simulator {
    pub loop_delay: u64,
    pub packet: Packet
}

impl Simulator {
    pub fn update(&mut self) -> &Packet {
        self.packet.altitude += 1.0;
        self.packet.velocity += 1.0;
        &self.packet
    }

    pub fn new() -> Self {
        let settings = match Settings::new() {
            Ok(settings) => {
                println!("Simulation configuration loaded successfully!");
                settings
            }
            Err(err) => {
                eprintln!("Failed to load configuration: {}", err);
                std::process::exit(1);
            }
        };

        Simulator {
            loop_delay: settings.sim_conf.loop_delay,
            packet: Packet {
                version: settings.ccsds_conf.version,
                r#type: settings.ccsds_conf.r#type,
                secondary_header: settings.ccsds_conf.secondary_header,
                apid: settings.ccsds_conf.apid,
                sequence_flags: settings.ccsds_conf.sequence_flags,
                sequence_count: settings.ccsds_conf.sequence_count,
                payload_length: settings.ccsds_conf.payload_length,
                altitude: settings.ccsds_conf.altitude,
                velocity: settings.ccsds_conf.velocity
            }
        }
    }
}
