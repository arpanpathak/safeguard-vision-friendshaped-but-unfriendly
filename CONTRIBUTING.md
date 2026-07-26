# 🤝 Contributing to Safeguard Vision

First off, thank you for caring about protecting people from emotional manipulation. This project exists because trauma survivors deserve tools that help them trust their own perception.

## 🧭 Roadmap

### Phase 1: Core Pipeline (MVP)
- [ ] **#[1]** Implement CUDA‑Oxide landmark regression kernel (`src/kernels/micro_expression.rs`)
- [ ] **#[2]** Wire up real AU extraction from MediaPipe/YOLOv8 face detection
- [ ] **#[3]** Integrate Whisper.cpp for on‑device STT (`src/nlp/transcript.rs`)
- [ ] **#[4]** Connect Mistral 7B (llama.cpp) for LLM pattern detection
- [ ] **#[5]** Implement companion band BLE driver (`src/physiology/stress.rs`)
- [ ] **#[6]** Stitch the real‑time loop in `main.rs`

### Phase 2: Accuracy & UX
- [ ] Temporal smoothing & hysteresis for risk scoring
- [ ] Per‑user baseline calibration
- [ ] React PWA companion app
- [ ] Decoy mode implementation

### Phase 3: Safety & Hardening
- [ ] Emergency wipe mechanism
- [ ] Encrypted session logging
- [ ] Biometric authentication
- [ ] Third‑party security audit

## 🧪 Development Workflow

1. **Pick a TODO** from the module you want to work on
2. **Assign yourself** by commenting on the issue
3. **Branch off `main`**: `git checkout -b feat/your-feature`
4. **Replace `todo!()` with real implementation**
5. **Add tests** (we use standard `#[cfg(test)]` mods)
6. **Run**: `cargo test && cargo clippy`
7. **Open a PR** with a clear description of what you changed

## 📏 Coding Standards

**PRs that do not adhere to our coding standards will be rejected.**

Read the full standards document: [CODING_STANDARDS.md](CODING_STANDARDS.md)

Key rules at a glance:
- **Functional over imperative** — prefer pure functions, avoid mutable state
- **Idiomatic Rust** — use enums, match, iterators, and the type system
- **Document everything** — every public function needs a doc comment
- **No unsafe blocks** without `// SAFETY:` justification
- **SOLID principles** — small interfaces, clear responsibilities
- **Formal verification** — property-based tests for critical math/logic
- **No dead code** — commented-out code gets rejected
- **Zero-cost abstractions** — no allocations on the hot path

## 🧑‍⚖️ Contributor Covenant

By participating, you agree to uphold a harassment‑free experience for everyone. Be excellent to each other.

---

<div align="center">
⭐ **Every contribution matters.** Even fixing a typo helps someone trust their gut.
</div>
