pub mod parserInfo;
pub mod parserHeader;
pub mod parserMotionData;
pub mod parserSessionData;
pub mod parserLapData;
pub mod parserEventData;
pub mod parserParticipantsData;
pub mod parserCarSetupData;
pub mod parserCarTelemetryData;
pub mod parserCarStatusData;

use serde_derive::{Serialize};

#[derive(Debug, Serialize)]
pub struct PacketResult {
    packetType: String,
    packetData: PacketData
}

pub struct Packet;

impl Packet {
    fn parse(input: &[u8], packet_id: &u8) -> PacketResult {
        match packet_id {
            0 => {
                let result = parserMotionData::parse_motion_data_packet(&input);
                let (_, packet) = result.unwrap();
                PacketResult {
                    packetType: "MotionData".to_string(),
                    packetData: PacketData::MotionData(packet)
                }
            }
            1 => {
                let result = parserSessionData::parse_session_data_packet(&input);
                let (_, packet) = result.unwrap();
                PacketResult {
                    packetType: "SessionData".to_string(),
                    packetData: PacketData::SessionData(packet)
                }
            }
            2 => {
                let result = parserLapData::parse_lap_data_packet(&input);
                let (_, packet) = result.unwrap();
                PacketResult {
                    packetType: "LapData".to_string(),
                    packetData: PacketData::LapData(packet)
                }
            }
            3 => {
                let result = parserEventData::parse_event_data_packet(&input);
                let (_, packet) = result.unwrap();
                PacketResult {
                    packetType: "EventData".to_string(),
                    packetData: PacketData::EventData(packet)
                }
            }
            4 => {
                let result = parserParticipantsData::parse_participants_data_packet(&input);
                let (_, packet) = result.unwrap();
                PacketResult {
                    packetType: "ParticipantsData".to_string(),
                    packetData: PacketData::ParticipantsData(packet)
                }
            }
            5 => {
                let result = parserCarSetupData::parse_car_setup_data_packet(&input);
                let (_, packet) = result.unwrap();
                PacketResult {
                    packetType: "CarSetupData".to_string(),
                    packetData: PacketData::CarSetupData(packet)
                }
            }
            6 => {
                let result = parserCarTelemetryData::parse_car_telemetry_data_packet(&input);
                let (_, packet) = result.unwrap();
                PacketResult {
                    packetType: "CarTelemetryData".to_string(),
                    packetData: PacketData::CarTelemetryData(packet)
                }
            }
            7 => {
                let result = parserCarStatusData::parse_car_status_data_packet(&input);
                let (_, packet) = result.unwrap();
                PacketResult {
                    packetType: "CarStatusData".to_string(),
                    packetData: PacketData::CarStatusData(packet)
                }
            }
            _ => panic!("Unknown packet type")
        }
    }
}

#[derive(Debug, Serialize)]
pub enum PacketData {
    MotionData(parserMotionData::PacketMotionData),
    SessionData(parserSessionData::PacketSessionData),
    LapData(parserLapData::PacketLapData),
    EventData(parserEventData::PacketEventData),
    ParticipantsData(parserParticipantsData::PacketParticipantsData),
    CarSetupData(parserCarSetupData::PacketCarSetupData),
    CarTelemetryData(parserCarTelemetryData::PacketCarTelemetryData),
    CarStatusData(parserCarStatusData::PacketCarStatusData),
}

pub fn parse_packet(input: &[u8]) -> PacketResult {
    let (_, packet_info) = parserInfo::get_packet_info(&input).unwrap();
    Packet::parse(&input, &packet_info.m_packetId)
}
