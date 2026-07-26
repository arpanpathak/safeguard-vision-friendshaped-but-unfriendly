//! 🗣️ NLP Module
//!
//! Speech transcription (Whisper STT wrapper) and love bombing pattern
//! detection combining rule-based keyword counting with local LLM inference.
//!
//! ## 📋 TODO
//!
//! - [ ] Integrate Whisper.cpp for on-device STT
//! - [ ] Optimize pattern matcher for real-time streaming
//! - [ ] Add support for multiple languages
//! - [ ] Write fuzz tests against known manipulation transcripts

pub mod transcript;
pub mod patterns;
