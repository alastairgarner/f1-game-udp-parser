use nom::number::complete::{le_u8, le_i8, le_u16, le_u32, le_f32};
use serde_derive::{Serialize};
use super::parserHeader::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct CarTelemetryData {
    pub m_speed: u16,
    pub m_throttle: f32,
    pub m_steer: f32,
    pub m_clutch: u8,
    pub m_gear: i8,
    pub m_engineRPM: u16,
    pub m_drs: u8,
    pub m_revLightsPercent: u8,
    pub m_brakesTemperature: Vec<u16>,
    pub m_tyresSurfaceTemperature: Vec<u16>,
    pub m_tyresInnerTemperature: Vec<u16>,
    pub m_engineTemperature: u16,
    pub m_tyresPressure: Vec<f32>,
    pub m_surfaceType: Vec<u8>
}

named!(pub parse_car_telemetry_data<&[u8], CarTelemetryData>,
    do_parse!(
        m_speed: le_u16 >>
        m_throttle: le_f32 >>
        m_steer: le_f32 >>
        m_clutch: le_u8 >>
        m_gear: le_i8 >>
        m_engineRPM: le_u16 >>
        m_drs: le_u8 >>
        m_revLightsPercent: le_u8 >>
        m_brakesTemperature: count!(le_u16, 4) >>
        m_tyresSurfaceTemperature: count!(le_u16, 4) >>
        m_tyresInnerTemperature: count!(le_u16, 4) >>
        m_engineTemperature: le_u16 >>
        m_tyresPressure: count!(le_f32, 4) >>
        m_surfaceType: count!(le_u8, 4) >>
        (CarTelemetryData {
            m_speed: m_speed,
            m_throttle: m_throttle,
            m_steer: m_steer,
            m_clutch: m_clutch,
            m_gear: m_gear,
            m_engineRPM: m_engineRPM,
            m_drs: m_drs,
            m_revLightsPercent: m_revLightsPercent,
            m_brakesTemperature: m_brakesTemperature,
            m_tyresSurfaceTemperature: m_tyresSurfaceTemperature,
            m_tyresInnerTemperature: m_tyresInnerTemperature,
            m_engineTemperature: m_engineTemperature,
            m_tyresPressure: m_tyresPressure,
            m_surfaceType: m_surfaceType
        })
    )
);

named!(pub parse_car_telemetry_datas<&[u8], Vec<CarTelemetryData>>,
    count!(parse_car_telemetry_data, 20)
);

#[derive(Debug, PartialEq, Serialize)]
pub struct PacketCarTelemetryData {
    pub m_header: PacketHeader,
    pub m_carTelemetryData: Vec<CarTelemetryData>,
    pub m_buttonStatus: u32
}

named!(pub parse_car_telemetry_data_packet<&[u8], PacketCarTelemetryData>,
    do_parse!(
        m_header: parse_header >>
        m_carTelemetryData: parse_car_telemetry_datas >>
        m_buttonStatus: le_u32 >>
        (PacketCarTelemetryData {
            m_header: m_header,
            m_carTelemetryData: m_carTelemetryData,
            m_buttonStatus: m_buttonStatus
        })
    )
);
