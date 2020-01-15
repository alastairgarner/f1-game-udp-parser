use nom::number::complete::{le_u8, le_f32};
use serde_derive::{Serialize};
use super::parserHeader::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct LapData {
    pub m_lastLapTime: f32,
    pub m_currentLapTime: f32,
    pub m_bestLapTime: f32,
    pub m_sector1Time: f32,
    pub m_sector2Time: f32,
    pub m_lapDistance: f32,
    pub m_totalDistance: f32,
    pub m_safetyCarDelta: f32,
    pub m_carPosition: u8,
    pub m_currentLapNum: u8,
    pub m_pitStatus: u8,
    pub m_sector: u8,
    pub m_currentLapInvalid: u8,
    pub m_penalties: u8,
    pub m_gridPosition: u8,
    pub m_driverStatus: u8,
    pub m_resultStatus: u8
}

named!(pub parse_lap_data<&[u8], LapData>,
    do_parse!(
        m_lastLapTime: le_f32 >>
        m_currentLapTime: le_f32 >>
        m_bestLapTime: le_f32 >>
        m_sector1Time: le_f32 >>
        m_sector2Time: le_f32 >>
        m_lapDistance: le_f32 >>
        m_totalDistance: le_f32 >>
        m_safetyCarDelta: le_f32 >>
        m_carPosition: le_u8 >>
        m_currentLapNum: le_u8 >>
        m_pitStatus: le_u8 >>
        m_sector: le_u8 >>
        m_currentLapInvalid: le_u8 >>
        m_penalties: le_u8 >>
        m_gridPosition: le_u8 >>
        m_driverStatus: le_u8 >>
        m_resultStatus: le_u8 >>
        (LapData {
            m_lastLapTime: m_lastLapTime,
            m_currentLapTime: m_currentLapTime,
            m_bestLapTime: m_bestLapTime,
            m_sector1Time: m_sector1Time,
            m_sector2Time: m_sector2Time,
            m_lapDistance: m_lapDistance,
            m_totalDistance: m_totalDistance,
            m_safetyCarDelta: m_safetyCarDelta,
            m_carPosition: m_carPosition,
            m_currentLapNum: m_currentLapNum,
            m_pitStatus: m_pitStatus,
            m_sector: m_sector,
            m_currentLapInvalid: m_currentLapInvalid,
            m_penalties: m_penalties,
            m_gridPosition: m_gridPosition,
            m_driverStatus: m_driverStatus,
            m_resultStatus: m_resultStatus
        })
    )
);

named!(pub parse_lap_datas<&[u8], Vec<LapData>>,
    count!(parse_lap_data, 20)
);

#[derive(Debug, PartialEq, Serialize)]
pub struct PacketLapData {
    pub m_header: PacketHeader,
    pub m_lapData: Vec<LapData>,
}

named!(pub parse_lap_data_packet<&[u8], PacketLapData>,
    do_parse!(
        m_header: parse_header >>
        m_lapData: parse_lap_datas >>
        (PacketLapData {
            m_header: m_header,
            m_lapData: m_lapData
        })
    )
);
