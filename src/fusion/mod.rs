//! ⚖️ Multimodal Fusion Module
//!
//! Combines vision, NLP, and physiology scores into a single risk indicator
//! using a weighted fusion approach. Maps the fused score to a severity level.
//!
//! ## 📋 TODO
//!
//! - [ ] Add temporal decay (recent scores weighted higher)
//! - [ ] Implement configurable per-user sensitivity
//! - [ ] Add calibration mode for new users
//! - [ ] Write property-based tests for fusion math

pub mod risk;
