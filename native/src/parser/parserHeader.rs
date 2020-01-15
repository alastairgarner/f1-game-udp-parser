use nom::number::complete::{le_u8, le_u16, le_u32, le_u64, le_f32};
use serde_derive::{Serialize};

#[derive(Debug, PartialEq, Serialize)]
pub struct PacketHeader {
    pub m_packetFormat: u16,
    pub m_gameMajorVersion: u8,
    pub m_gameMinorVersion: u8,
    pub m_packetVersion: u8,
    pub m_packetId: u8,
    pub m_sessionUID: u64,
    pub m_sessionTime: f32,
    pub m_frameIdentifier: u32,
    pub m_playerCarIndex: u8
}

named!(pub parse_header<&[u8], PacketHeader>,
    do_parse!(
        m_packetFormat: le_u16 >>
        m_gameMajorVersion: le_u8 >>
        m_gameMinorVersion: le_u8 >>
        m_packetVersion: le_u8 >>
        m_packetId: le_u8 >>
        m_sessionUID: le_u64 >>
        m_sessionTime: le_f32 >>
        m_frameIdentifier: le_u32 >>
        m_playerCarIndex : le_u8 >>
        (PacketHeader {
            m_packetFormat: m_packetFormat,
            m_gameMajorVersion: m_gameMajorVersion,
            m_gameMinorVersion: m_gameMinorVersion,
            m_packetVersion: m_packetVersion,
            m_packetId: m_packetId,
            m_sessionUID: m_sessionUID,
            m_sessionTime: m_sessionTime,
            m_frameIdentifier: m_frameIdentifier,
            m_playerCarIndex: m_playerCarIndex
        })
    )
);
