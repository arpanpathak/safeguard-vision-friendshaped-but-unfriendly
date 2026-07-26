//! # 🔀 Expression Incongruence Detection
//!
//! Detects mismatches between facial expressions — e.g., smiling (AU12)
//! while frowning (AU4) — which can indicate emotional masking or
//! manipulation.
//!
//! ## TODO
//!
//! - [ ] Add temporal incongruence (expression changes too fast)
//! - [ ] Add asymmetry detection (one side of face more active)
//! - [ ] Validate against psychology literature on masking

#![allow(unused_variables, dead_code)]

use super::action_units::{ActionUnit, ActionUnitProfile};

/// Detects incongruence between facial expressions.
///
/// Returns a score in [0, 1] where 1.0 means high incongruence.
pub fn detect_expression_incongruence(au: &ActionUnitProfile) -> f32 {
    todo!("Implement expression incongruence scoring logic");
}
