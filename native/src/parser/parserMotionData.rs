use nom::number::complete::{le_i16, le_f32};
use serde_derive::{Serialize};
use super::parserHeader::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct CarMotionData {
    pub m_worldPositionX: f32,
    pub m_worldPositionY: f32,
    pub m_worldPositionZ: f32,
    pub m_worldVelocityX: f32,
    pub m_worldVelocityY: f32,
    pub m_worldVelocityZ: f32,
    pub m_worldForwardDirX: i16,
    pub m_worldForwardDirY: i16,
    pub m_worldForwardDirZ: i16,
    pub m_worldRightDirX: i16,
    pub m_worldRightDirY: i16,
    pub m_worldRightDirZ: i16,
    pub m_gForceLateral: f32,
    pub m_gForceLongitudinal: f32,
    pub m_gForceVertical: f32,
    pub m_yaw: f32,
    pub m_pitch: f32,
    pub m_roll: f32,
}

named!(pub parse_motion_data<&[u8], CarMotionData>,
    do_parse!(
        m_worldPositionX: le_f32 >>
        m_worldPositionY: le_f32 >>
        m_worldPositionZ: le_f32 >>
        m_worldVelocityX: le_f32 >>
        m_worldVelocityY: le_f32 >>
        m_worldVelocityZ: le_f32 >>
        m_worldForwardDirX: le_i16 >>
        m_worldForwardDirY: le_i16 >>
        m_worldForwardDirZ: le_i16 >>
        m_worldRightDirX: le_i16 >>
        m_worldRightDirY: le_i16 >>
        m_worldRightDirZ: le_i16 >>
        m_gForceLateral: le_f32 >>
        m_gForceLongitudinal: le_f32 >>
        m_gForceVertical: le_f32 >>
        m_yaw: le_f32 >>
        m_pitch: le_f32 >>
        m_roll: le_f32 >>
        (CarMotionData {
            m_worldPositionX: m_worldPositionX,
            m_worldPositionY: m_worldPositionY,
            m_worldPositionZ: m_worldPositionZ,
            m_worldVelocityX: m_worldVelocityX,
            m_worldVelocityY: m_worldVelocityY,
            m_worldVelocityZ: m_worldVelocityZ,
            m_worldForwardDirX: m_worldForwardDirX,
            m_worldForwardDirY: m_worldForwardDirY,
            m_worldForwardDirZ: m_worldForwardDirZ,
            m_worldRightDirX: m_worldRightDirX,
            m_worldRightDirY: m_worldRightDirY,
            m_worldRightDirZ: m_worldRightDirZ,
            m_gForceLateral: m_gForceLateral,
            m_gForceLongitudinal: m_gForceLongitudinal,
            m_gForceVertical: m_gForceVertical,
            m_yaw: m_yaw,
            m_pitch: m_pitch,
            m_roll: m_roll,
        })
    )
);

named!(pub parse_motion_datas<&[u8], Vec<CarMotionData>>,
    count!(parse_motion_data, 20)
);

#[derive(Debug, PartialEq, Serialize)]
pub struct PacketMotionData {
    pub m_header: PacketHeader,
    pub m_carMotionData: Vec<CarMotionData>,
    pub m_suspensionPosition: Vec<f32>,
    pub m_suspensionVelocity: Vec<f32>,
    pub m_suspensionAcceleration: Vec<f32>,
    pub m_wheelSpeed: Vec<f32>,
    pub m_wheelSlip: Vec<f32>,
    pub m_localVelocityX: f32,
    pub m_localVelocityY: f32,
    pub m_localVelocityZ: f32,
    pub m_angularVelocityX: f32,
    pub m_angularVelocityY: f32,
    pub m_angularVelocityZ: f32,
    pub m_angularAccelerationX: f32,
    pub m_angularAccelerationY: f32,
    pub m_angularAccelerationZ: f32,
    pub m_frontWheelsAngle: f32
}

named!(pub parse_motion_data_packet<&[u8], PacketMotionData>,
    do_parse!(
        m_header: parse_header >>
        m_carMotionData: parse_motion_datas >>
        m_suspensionPosition: count!(le_f32, 4) >>
        m_suspensionVelocity: count!(le_f32, 4) >>
        m_suspensionAcceleration: count!(le_f32, 4) >>
        m_wheelSpeed: count!(le_f32, 4) >>
        m_wheelSlip: count!(le_f32, 4) >>
        m_localVelocityX: le_f32 >>
        m_localVelocityY: le_f32 >>
        m_localVelocityZ: le_f32 >>
        m_angularVelocityX: le_f32 >>
        m_angularVelocityY: le_f32 >>
        m_angularVelocityZ: le_f32 >>
        m_angularAccelerationX: le_f32 >>
        m_angularAccelerationY: le_f32 >>
        m_angularAccelerationZ: le_f32 >>
        m_frontWheelsAngle: le_f32 >>
        (PacketMotionData {
            m_header: m_header,
            m_carMotionData: m_carMotionData,
            m_suspensionPosition: m_suspensionPosition,
            m_suspensionVelocity: m_suspensionVelocity,
            m_suspensionAcceleration: m_suspensionAcceleration,
            m_wheelSpeed: m_wheelSpeed,
            m_wheelSlip: m_wheelSlip,
            m_localVelocityX: m_localVelocityX,
            m_localVelocityY: m_localVelocityY,
            m_localVelocityZ: m_localVelocityZ,
            m_angularVelocityX: m_angularVelocityX,
            m_angularVelocityY: m_angularVelocityY,
            m_angularVelocityZ: m_angularVelocityZ,
            m_angularAccelerationX: m_angularAccelerationX,
            m_angularAccelerationY: m_angularAccelerationY,
            m_angularAccelerationZ: m_angularAccelerationZ,
            m_frontWheelsAngle: m_frontWheelsAngle
        })
    )
);
