use nom::number::complete::{le_u8, le_f32};
use serde_derive::{Serialize};
use std::str;
use super::parserHeader::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct EventDataDetails {
    pub vehicleIdx: u8,
    pub lapTime: Option<f32>
}

named!(pub parse_event_details<&[u8], EventDataDetails>,
    do_parse!(
        vehicleIdx: le_u8 >>
        lapTime: opt!(le_f32) >>
        (EventDataDetails {
            vehicleIdx: vehicleIdx,
            lapTime: lapTime
        })
    )
);

#[derive(Debug, PartialEq, Serialize)]
pub struct PacketEventData {
    pub m_header: PacketHeader,
    pub m_eventStringCode: String,
    pub m_eventDetails: Option<EventDataDetails>
}

named!(pub parse_event_data_packet<&[u8], PacketEventData>,
    do_parse!(
        m_header: parse_header >>
        m_eventStringCode: map_res!(take!(4), str::from_utf8) >>
        m_eventDetails: opt!(parse_event_details) >>
        (PacketEventData {
            m_header: m_header,
            m_eventStringCode: m_eventStringCode.to_string(),
            m_eventDetails: m_eventDetails
        })
    )
);
