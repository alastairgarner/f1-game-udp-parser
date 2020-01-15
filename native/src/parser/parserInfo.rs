use nom::{peek, IResult};
use nom::number::complete::{le_u8, le_u16};

#[derive(Debug, PartialEq)]
pub struct PacketInfo {
    pub m_packetFormat: u16,
    pub m_gameMajorVersion: u8,
    pub m_gameMinorVersion: u8,
    pub m_packetVersion: u8,
    pub m_packetId: u8,
}

named!(pub parser_packet_info<&[u8], PacketInfo>,
    peek!(
        do_parse!(
            m_packetFormat: le_u16 >>
            m_gameMajorVersion: le_u8 >>
            m_gameMinorVersion: le_u8 >>
            m_packetVersion: le_u8 >>
            m_packetId: le_u8 >>
            (PacketInfo {
                m_packetFormat: m_packetFormat,
                m_gameMajorVersion: m_gameMajorVersion,
                m_gameMinorVersion: m_gameMinorVersion,
                m_packetVersion: m_packetVersion,
                m_packetId: m_packetId
            })
        )
    )
);

pub fn get_packet_info(input: &[u8]) -> IResult<&[u8], PacketInfo> {
    match parser_packet_info(input) {
        IResult::Ok((remaining, packet_info)) => IResult::Ok((remaining, packet_info)),
        _e => panic!("Unable to parse")
    }
}
