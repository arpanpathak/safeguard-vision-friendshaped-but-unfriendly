//! # 🎤 Speech Transcription
//!
//! Wrapper around Whisper STT for real-time speech-to-text on edge devices.
//!
//! ## TODO
//!
//! - [ ] Integrate whisper.cpp bindings
//! - [ ] Implement streaming (sliding window) transcription
//! - [ ] Handle speaker diarization (who said what)
//! - [ ] Add vad (voice activity detection) for power efficiency

#![allow(unused_variables, dead_code)]

/// Transcribes an audio buffer to text using Whisper.
///
/// In production this calls whisper.cpp via FFI.
pub fn transcribe_audio(audio_buffer: &[f32], sample_rate: u32) -> Result<String, String> {
    todo!("Integrate whisper.cpp for on-device STT");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_transcribe_stub() {
        let dummy_audio = vec![0.0_f32; 16000];
        let _ = transcribe_audio(&dummy_audio, 16000).unwrap();
    }
}
