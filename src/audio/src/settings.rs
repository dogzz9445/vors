use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use settings_schema::{DictionaryDefault, NamedEntry, SettingsSchema, Switch, SwitchDefault};

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
pub enum AudioDeviceId {
    Default,
    Name(String),
    #[schema(min = 1, gui = "UpDown")]
    Index(u64),
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioBufferingConfig {
    #[schema(min = 0, max = 200)]
    pub average_buffering_ms: u64,

    #[schema(advanced, min = 1, max = 20)]
    pub batch_ms: u64,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameAudioDesc {
    #[schema(placeholder = "device_dropdown")]
    //
    #[schema(advanced)]
    pub device_id: AudioDeviceId,
    pub mute_when_streaming: bool,
    pub buffering_config: AudioBufferingConfig,
}

// Note: sample rate is a free parameter for microphone, because both server and client supports
// resampling. In contrary, for game audio, the server does not support resampling.
#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MicrophoneDesc {
    #[schema(placeholder = "input_device_dropdown")]
    //
    #[schema(advanced)]
    pub input_device_id: AudioDeviceId,

    #[schema(placeholder = "output_device_dropdown")]
    //
    #[cfg(not(target_os = "linux"))]
    #[schema(advanced)]
    pub output_device_id: AudioDeviceId,

    pub buffering_config: AudioBufferingConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
pub enum LinuxAudioBackend {
    Alsa,
    Jack,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioSection {
    #[schema(advanced)]
    pub linux_backend: LinuxAudioBackend,

    pub game_audio: Switch<GameAudioDesc>,

    pub microphone: Switch<MicrophoneDesc>,
}
