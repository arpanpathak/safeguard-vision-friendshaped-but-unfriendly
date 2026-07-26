//! 🦀 CUDA‑Oxide GPU Kernels
//!
//! GPU kernels written in **pure Rust** that compile to PTX via NVIDIA's
//! CUDA‑Oxide toolchain. These perform the heavy lifting for real‑time
//! facial landmark regression and feature extraction on edge hardware.
//!
//! ## 📋 TODO
//!
//! - [ ] Implement micro-expression heatmap regression kernel
//! - [ ] Add shared memory optimization for landmark voting
//! - [ ] Write warp-level primitives for AU aggregation
//! - [ ] Benchmark against equivalent C++ CUDA kernels
//! - [ ] Document PTX output size and register pressure

pub mod micro_expression;
