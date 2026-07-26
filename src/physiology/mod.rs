//! 💓 Physiology Module
//!
//! Processes companion band sensor data — HRV, GSR, skin temperature,
//! and Blood Volume Pulse (BVP) — to compute a real-time stress score.
//!
//! ## 📋 TODO
//!
//! - [ ] Implement BLE driver for companion band
//! - [ ] Add HRV frequency-domain analysis (LF/HF ratio)
//! - [ ] Implement baseline calibration per user
//! - [ ] Write tests with real physiological stress datasets

pub mod stress;
