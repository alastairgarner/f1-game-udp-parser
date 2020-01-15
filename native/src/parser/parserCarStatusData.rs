use nom::number::complete::{le_u8, le_i8, le_u16, le_f32};
use serde_derive::{Serialize};
use super::parserHeader::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct CarStatusData {
    pub m_tractionControl: u8,
    pub m_antiLockBrakes: u8,
    pub m_fuelMix: u8,
    pub m_frontBrakeBias: u8,
    pub m_pitLimiterStatus: u8,
    pub m_fuelInTank: f32,
    pub m_fuelCapacity: f32,
    pub m_fuelRemainingLaps: f32,
    pub m_maxRPM: u16,
    pub m_idleRPM: u16,
    pub m_maxGears: u8,
    pub m_drsAllowed: u8,
    pub m_tyresWear: Vec<u8>,
    pub m_actualTyreCompound: u8,
    pub m_tyreVisualCompound: u8,
    pub m_tyresDamage: Vec<u8>,
    pub m_frontLeftWingDamage: u8,
    pub m_frontRightWingDamage: u8,
    pub m_rearWingDamage: u8,
    pub m_engineDamage: u8,
    pub m_gearBoxDamage: u8,
    pub m_vehicleFiaFlags: i8,
    pub m_ersStoreEnergy: f32,
    pub m_ersDeployMode: u8,
    pub m_ersHarvestedThisLapMGUK: f32,
    pub m_ersHarvestedThisLapMGUH: f32,
    pub m_ersDeployedThisLap: f32
}

named!(pub parse_car_status_data<&[u8], CarStatusData>,
    do_parse!(
        m_tractionControl: le_u8 >>
        m_antiLockBrakes: le_u8 >>
        m_fuelMix: le_u8 >>
        m_frontBrakeBias: le_u8 >>
        m_pitLimiterStatus: le_u8 >>
        m_fuelInTank: le_f32 >>
        m_fuelCapacity: le_f32 >>
        m_fuelRemainingLaps: le_f32 >>
        m_maxRPM: le_u16 >>
        m_idleRPM: le_u16 >>
        m_maxGears: le_u8 >>
        m_drsAllowed: le_u8 >>
        m_tyresWear: count!(le_u8, 4) >>
        m_actualTyreCompound: le_u8 >>
        m_tyreVisualCompound: le_u8 >>
        m_tyresDamage: count!(le_u8, 4) >>
        m_frontLeftWingDamage: le_u8 >>
        m_frontRightWingDamage: le_u8 >>
        m_rearWingDamage: le_u8 >>
        m_engineDamage: le_u8 >>
        m_gearBoxDamage: le_u8 >>
        m_vehicleFiaFlags: le_i8 >>
        m_ersStoreEnergy: le_f32 >>
        m_ersDeployMode: le_u8 >>
        m_ersHarvestedThisLapMGUK: le_f32 >>
        m_ersHarvestedThisLapMGUH: le_f32 >>
        m_ersDeployedThisLap: le_f32 >>
        (CarStatusData {
            m_tractionControl: m_tractionControl,
            m_antiLockBrakes: m_antiLockBrakes,
            m_fuelMix: m_fuelMix,
            m_frontBrakeBias: m_frontBrakeBias,
            m_pitLimiterStatus: m_pitLimiterStatus,
            m_fuelInTank: m_fuelInTank,
            m_fuelCapacity: m_fuelCapacity,
            m_fuelRemainingLaps: m_fuelRemainingLaps,
            m_maxRPM: m_maxRPM,
            m_idleRPM: m_idleRPM,
            m_maxGears: m_maxGears,
            m_drsAllowed: m_drsAllowed,
            m_tyresWear: m_tyresWear,
            m_actualTyreCompound: m_actualTyreCompound,
            m_tyreVisualCompound: m_tyreVisualCompound,
            m_tyresDamage: m_tyresDamage,
            m_frontLeftWingDamage: m_frontLeftWingDamage,
            m_frontRightWingDamage: m_frontRightWingDamage,
            m_rearWingDamage: m_rearWingDamage,
            m_engineDamage: m_engineDamage,
            m_gearBoxDamage: m_gearBoxDamage,
            m_vehicleFiaFlags: m_vehicleFiaFlags,
            m_ersStoreEnergy: m_ersStoreEnergy,
            m_ersDeployMode: m_ersDeployMode,
            m_ersHarvestedThisLapMGUK: m_ersHarvestedThisLapMGUK,
            m_ersHarvestedThisLapMGUH: m_ersHarvestedThisLapMGUH,
            m_ersDeployedThisLap: m_ersDeployedThisLap
        })
    )
);

named!(pub parse_car_status_datas<&[u8], Vec<CarStatusData>>,
    count!(parse_car_status_data, 20)
);

#[derive(Debug, PartialEq, Serialize)]
pub struct PacketCarStatusData {
    pub m_header: PacketHeader,
    pub m_carStatusData: Vec<CarStatusData>
}

named!(pub parse_car_status_data_packet<&[u8], PacketCarStatusData>,
    do_parse!(
        m_header: parse_header >>
        m_carStatusData: parse_car_status_datas >>
        (PacketCarStatusData {
            m_header: m_header,
            m_carStatusData: m_carStatusData
        })
    )
);
