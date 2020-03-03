use nom::number::complete::{le_u8};
use serde_derive::{Serialize};
use std::str;
use super::parserHeader::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct ParticipantData {
    pub m_aiControlled: u8,
    pub m_driverId: u8,
    pub m_teamId: u8,
    pub m_raceNumber: u8,
    pub m_nationality: u8,
    pub m_name: String,
    pub m_yourTelemetry: u8
}

named!(pub parse_participant<&[u8], ParticipantData>,
    do_parse!(
        m_aiControlled: le_u8 >>
        m_driverId: le_u8 >>
        m_teamId: le_u8 >>
        m_raceNumber: le_u8 >>
        m_nationality: le_u8 >>
        m_name: map_res!(take!(48), str::from_utf8) >>
        m_yourTelemetry: le_u8 >>
        (ParticipantData {
            m_aiControlled: m_aiControlled,
            m_driverId: m_driverId,
            m_teamId: m_teamId,
            m_raceNumber: m_raceNumber,
            m_nationality: m_nationality,
            m_name: m_name.trim_matches(char::from(0)).to_string(),
            m_yourTelemetry: m_yourTelemetry
        })
    )
);

named!(pub parse_participants<&[u8], Vec<ParticipantData>>,
    count!(parse_participant, 20)
);

#[derive(Debug, PartialEq, Serialize)]
pub struct PacketParticipantsData {
    pub m_header: PacketHeader,
    pub m_numActiveCars: u8,
    pub m_participants: Vec<ParticipantData>,
}

named!(pub parse_participants_data_packet<&[u8], PacketParticipantsData>,
    do_parse!(
        m_header: parse_header >>
        m_numActiveCars: le_u8 >>
        m_participants: parse_participants >>
        (PacketParticipantsData {
            m_header: m_header,
            m_numActiveCars: m_numActiveCars,
            m_participants: m_participants
        })
    )
);
