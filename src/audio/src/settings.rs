use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
pub enum AudioDeviceId {
    Default,
    Name(String),
    Index(u64),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioBufferingConfig {
    pub average_buffering_ms: u64,
    pub batch_ms: u64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameAudioDesc {
    pub device_id: AudioDeviceId,
    pub mute_when_streaming: bool,
    pub buffering_config: AudioBufferingConfig,
}

// Note: sample rate is a free parameter for microphone, because both server and client supports
// resampling. In contrary, for game audio, the server does not support resampling.
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MicrophoneDesc {
    pub input_device_id: AudioDeviceId,

    #[cfg(not(target_os = "linux"))]
    pub output_device_id: AudioDeviceId,

    pub buffering_config: AudioBufferingConfig,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
pub enum LinuxAudioBackend {
    Alsa,
    Jack,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioSection {
    pub linux_backend: LinuxAudioBackend,

    pub game_audio: GameAudioDesc,

    pub microphone: MicrophoneDesc,
}
