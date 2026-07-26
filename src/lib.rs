//! # 😎 Safeguard Vision
//!
//! Open‑source AI‑powered smart glasses system that detects **love bombing** and
//! emotional manipulation in **real time** during conversations.
//!
//! ## 🧠 Philosophy
//!
//! - **Never profile the other person.** Only profile the user's response to them.
//! - **On‑device by default.** No cloud dependency.
//! - **Privacy is non‑negotiable.** Real‑time inference only, no recording, no storage.
//!
//! ## 📦 Modules
//!
//! | Module | Path | Purpose |
//! |--------|------|---------|
//! | `kernels` | `src/kernels/` | CUDA‑Oxide GPU kernels (Rust → PTX) |
//! | `vision` | `src/vision/` | Facial landmark extraction, AU scoring, incongruence detection |
//! | `nlp` | `src/nlp/` | Speech transcription & love bombing pattern matching |
//! | `physiology` | `src/physiology/` | HRV / GSR / skin temp stress scoring |
//! | `fusion` | `src/fusion/` | Multimodal weighted risk fusion engine |
//! | `alert` | `src/alert/` | Haptic, LED, and log notification system |
//!
//! ## 🚧 Status
//!
//! **Pre-alpha / Skeleton.** All modules are stubbed with `todo!()`.
//! See individual module docs for implementation roadmaps.

pub mod kernels;
pub mod vision;
pub mod nlp;
pub mod physiology;
pub mod fusion;
pub mod alert;
