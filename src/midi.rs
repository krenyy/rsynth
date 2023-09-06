#[derive(Debug)]
pub enum Message {
    ChannelMessage {
        channel: u8,
        kind: ChannelMessageKind,
    },
    SystemMessage {
        kind: SystemMessageKind,
    },
}

#[derive(Debug)]
pub enum ChannelMessageKind {
    NoteOff { key_number: u8, velocity: u8 },
    NoteOn { key_number: u8, velocity: u8 },
    PolyphonicKeyPressure,
    ControlChange,
    ProgramChange,
    ChannelPressure,
    PitchBend,
}

#[derive(Debug)]
pub enum SystemMessageKind {
    ActiveSensing,
}

fn note_number_to_human(num: u8) -> String {
    format!(
        "{}{}",
        match num % 12 {
            0 => "C",
            1 => "C#/Db",
            2 => "D",
            3 => "D#/Eb",
            4 => "E",
            5 => "F",
            6 => "F#/Gb",
            7 => "G",
            8 => "G#/Ab",
            9 => "A",
            10 => "A#/Bb",
            11 => "B",
            _ => unreachable!(),
        },
        num / 12
    )
}

#[derive(Debug)]
pub struct Midi {
    pub message: Message,
    pub channel: u8,
}

#[derive(Debug)]
pub enum ParseError {
    NoData,
    ByteSequenceTooLong,
    InvalidStatusByte,
    UnimplementedStatusByte(u8),
}

impl TryFrom<&[u8]> for Midi {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Err(ParseError::NoData);
        }
        if value.len() > 3 {
            return Err(ParseError::ByteSequenceTooLong);
        }

        let status = value[0];
        if status >> 7 != 1 {
            return Err(ParseError::InvalidStatusByte);
        }

        let message = match status >> 4 {
            (0x8..=0xE) => Message::ChannelMessage {
                channel: status & 0xF,
                kind: match status >> 4 {
                    0x8 => ChannelMessageKind::NoteOff {
                        key_number: value[1],
                        velocity: value[2],
                    },
                    0x9 => ChannelMessageKind::NoteOn {
                        key_number: value[1],
                        velocity: value[2],
                    },
                    0xA => ChannelMessageKind::PolyphonicKeyPressure,
                    0xB => ChannelMessageKind::ControlChange,
                    0xC => ChannelMessageKind::ProgramChange,
                    0xD => ChannelMessageKind::ChannelPressure,
                    0xE => ChannelMessageKind::PitchBend,
                    _ => return Err(ParseError::UnimplementedStatusByte(status)),
                },
            },
            0xF => Message::SystemMessage {
                kind: match status & 0xF {
                    0xE => SystemMessageKind::ActiveSensing,
                    _ => return Err(ParseError::UnimplementedStatusByte(status)),
                },
            },
            _ => return Err(ParseError::UnimplementedStatusByte(status)),
        };
        let channel = status & 0b1111;

        Ok(Self { message, channel })
    }
}
