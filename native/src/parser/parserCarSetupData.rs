use nom::number::complete::{le_u8, le_f32};
use serde_derive::{Serialize};
use super::parserHeader::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct CarSetupData {
    pub m_frontWing: u8,
    pub m_rearWing: u8,
    pub m_onThrottle: u8,
    pub m_offThrottle: u8,
    pub m_frontCamber: f32,
    pub m_rearCamber: f32,
    pub m_frontToe: f32,
    pub m_rearToe: f32,
    pub m_frontSuspension: u8,
    pub m_rearSuspension: u8,
    pub m_frontAntiRollBar: u8,
    pub m_rearAntiRollBar: u8,
    pub m_frontSuspensionHeight: u8,
    pub m_rearSuspensionHeight: u8,
    pub m_brakePressure: u8,
    pub m_brakeBias: u8,
    pub m_frontTyrePressure: f32,
    pub m_rearTyrePressure: f32,
    pub m_ballast: u8,
    pub m_fuelLoad: f32
}

named!(pub parse_car_setup_data<&[u8], CarSetupData>,
    do_parse!(
        m_frontWing: le_u8 >>
        m_rearWing: le_u8 >>
        m_onThrottle: le_u8 >>
        m_offThrottle: le_u8 >>
        m_frontCamber: le_f32 >>
        m_rearCamber: le_f32 >>
        m_frontToe: le_f32 >>
        m_rearToe: le_f32 >>
        m_frontSuspension: le_u8 >>
        m_rearSuspension: le_u8 >>
        m_frontAntiRollBar: le_u8 >>
        m_rearAntiRollBar: le_u8 >>
        m_frontSuspensionHeight: le_u8 >>
        m_rearSuspensionHeight: le_u8 >>
        m_brakePressure: le_u8 >>
        m_brakeBias: le_u8 >>
        m_frontTyrePressure: le_f32 >>
        m_rearTyrePressure: le_f32 >>
        m_ballast: le_u8 >>
        m_fuelLoad: le_f32 >>
        (CarSetupData {
            m_frontWing: m_frontWing,
            m_rearWing: m_rearWing,
            m_onThrottle: m_onThrottle,
            m_offThrottle: m_offThrottle,
            m_frontCamber: m_frontCamber,
            m_rearCamber: m_rearCamber,
            m_frontToe: m_frontToe,
            m_rearToe: m_rearToe,
            m_frontSuspension: m_frontSuspension,
            m_rearSuspension: m_rearSuspension,
            m_frontAntiRollBar: m_frontAntiRollBar,
            m_rearAntiRollBar: m_rearAntiRollBar,
            m_frontSuspensionHeight: m_frontSuspensionHeight,
            m_rearSuspensionHeight: m_rearSuspensionHeight,
            m_brakePressure: m_brakePressure,
            m_brakeBias: m_brakeBias,
            m_frontTyrePressure: m_frontTyrePressure,
            m_rearTyrePressure: m_rearTyrePressure,
            m_ballast: m_ballast,
            m_fuelLoad: m_fuelLoad
        })
    )
);

named!(pub parse_car_setup_datas<&[u8], Vec<CarSetupData>>,
    count!(parse_car_setup_data, 20)
);

#[derive(Debug, PartialEq, Serialize)]
pub struct PacketCarSetupData {
    pub m_header: PacketHeader,
    pub m_carSetups: Vec<CarSetupData>,
}

named!(pub parse_car_setup_data_packet<&[u8], PacketCarSetupData>,
    do_parse!(
        m_header: parse_header >>
        m_carSetups: parse_car_setup_datas >>
        (PacketCarSetupData {
            m_header: m_header,
            m_carSetups: m_carSetups
        })
    )
);
