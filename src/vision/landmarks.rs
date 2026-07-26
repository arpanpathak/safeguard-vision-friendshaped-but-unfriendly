//! # 👤 Facial Landmark Extraction
//!
//! Extracts 68 facial landmarks (x, y) from a normalized grayscale frame
//! using a heatmap regression approach on the GPU.
//!
//! ## TODO
//!
//! - [ ] Replace dummy circle heuristic with real GPU kernel call
//! - [ ] Add confidence scores per landmark
//! - [ ] Support profile/occluded face handling

#![allow(unused_variables, dead_code)]

use crate::kernels::micro_expression::NUM_LANDMARKS;

/// A single facial landmark coordinate, normalized to [0, 1].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Landmark {
    pub x: f32,
    pub y: f32,
}

/// A full set of 68 facial landmarks.
#[derive(Debug, Clone)]
pub struct FaceLandmarks {
    pub points: [Landmark; NUM_LANDMARKS],
}

impl FaceLandmarks {
    /// Creates a new empty set (for initialisation).
    pub fn new() -> Self {
        Self {
            points: [Landmark { x: 0.0, y: 0.0 }; NUM_LANDMARKS],
        }
    }
}

impl Default for FaceLandmarks {
    fn default() -> Self {
        Self::new()
    }
}

/// Extracts facial landmarks from a grayscale frame.
///
/// In production this calls the CUDA‑Oxide kernel. Currently returns
/// dummy data as a placeholder.
pub fn extract_landmarks(frame: &[f32], width: usize, height: usize) -> Result<FaceLandmarks, String> {
    todo!("Replace dummy heuristic with real GPU kernel invocation");
}
