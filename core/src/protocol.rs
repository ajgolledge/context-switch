//! Low-level serializable types that are used in the context-switch protocol and internal
//! service interfaces.

use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{
    AudioConsumer, AudioMsgConsumer, AudioMsgProducer, AudioProducer, audio_channel,
    audio_msg_channel,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioFormat {
    pub channels: u16,
    pub sample_rate: u32,
}

impl AudioFormat {
    pub fn new(channels: u16, sample_rate: u32) -> Self {
        Self {
            channels,
            sample_rate,
        }
    }

    pub fn duration(&self, no_samples: usize) -> Duration {
        let mono_sample_count = no_samples / self.channels as usize;
        Duration::from_secs_f64(mono_sample_count as f64 / self.sample_rate as f64)
    }

    pub fn new_channel(&self) -> (AudioProducer, AudioConsumer) {
        audio_channel(*self)
    }

    pub fn new_msg_channel(&self) -> (AudioMsgProducer, AudioMsgConsumer) {
        audio_msg_channel(*self)
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum InputModality {
    Audio { format: AudioFormat },
    Text,
}

impl InputModality {
    pub fn can_receive_audio(&self, input_format: AudioFormat) -> bool {
        matches!(self, InputModality::Audio { format } if *format == input_format)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum OutputModality {
    Audio { format: AudioFormat },
    Text,
    InterimText,
}
