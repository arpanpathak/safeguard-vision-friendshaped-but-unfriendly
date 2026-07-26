//! 👁️ Vision Module
//!
//! Facial landmark extraction, Action Unit (AU) intensity scoring, and
//! expression incongruence detection — the visual pipeline for
//! love bombing detection.
//!
//! ## 📋 TODO
//!
//! - [ ] Wire up real MediaPipe / YOLOv8 face detection
//! - [ ] Replace dummy heuristics with actual neural inference
//! - [ ] Add temporal smoothing across frames
//! - [ ] Profile performance on Jetson Orin Nano
//! - [ ] Write integration tests with real camera feed

pub mod landmarks;
pub mod action_units;
pub mod incongruence;
