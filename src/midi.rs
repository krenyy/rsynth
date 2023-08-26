use std::fmt;

#[derive(Debug)]
pub enum MidiMessage {
    NoteOff { key_number: u8, velocity: u8 },
    NoteOn { key_number: u8, velocity: u8 },
    PolyphonicKeyPressure,
    ControlChange,
    ProgramChange,
    ChannelPressure,
    PitchBend,
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

impl fmt::Display for MidiMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MidiMessage::NoteOff {
                key_number,
                velocity,
            } => write!(
                f,
                "NoteOff {}: {velocity}",
                note_number_to_human(*key_number)
            ),
            MidiMessage::NoteOn {
                key_number,
                velocity,
            } => write!(
                f,
                "NoteOn {}: {velocity}",
                note_number_to_human(*key_number)
            ),
            _ => write!(f, "{self:?}"),
        }
    }
}

#[derive(Debug)]
pub struct Midi {
    pub message: MidiMessage,
    pub channel: u8,
}

#[derive(Debug)]
pub enum ParseError {
    NoData,
    ByteSequenceTooLong,
    InvalidStatusByte,
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
            0x8 => MidiMessage::NoteOff {
                key_number: value[1],
                velocity: value[2],
            },
            0x9 => MidiMessage::NoteOn {
                key_number: value[1],
                velocity: value[2],
            },
            0xA => MidiMessage::PolyphonicKeyPressure,
            0xB => MidiMessage::ControlChange,
            0xC => MidiMessage::ProgramChange,
            0xD => MidiMessage::ChannelPressure,
            0xE => MidiMessage::PitchBend,
            _ => unreachable!(),
        };
        let channel = status & 0b1111;

        Ok(Self { message, channel })
    }
}
