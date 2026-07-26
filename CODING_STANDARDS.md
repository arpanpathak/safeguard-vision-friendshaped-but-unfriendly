# 📐 Coding Standards & Best Practices

> *"Reducing the distance between userspace and kernel space through great systems programming abstractions that don't leak."*

Every line of code in this project carries a responsibility — to the trauma survivors who will trust their safety to this system, to the contributors who build upon it, and to the open-source ethos that protects it from proprietary appropriation.

**PRs that do not adhere to these standards will be rejected.** Full stop.

---

## 🧠 Core Philosophy

### Userspace ↔ Kernel Space Continuum

As systems programmers, our job is to **build abstractions that don't leak**. Every layer — from the GPU kernel to the alert notification — should be:

- **Correct** — formally verifiable where possible
- **Performant** — zero-cost unless explicitly traded off
- **Transparent** — the abstraction hides complexity, not behavior

> *"A great abstraction lets you forget what's underneath. A leaking abstraction forces you to remember everything."*

### Safety First

- **No `unsafe` without `// SAFETY:`** — every unsafe block must have a justification that a reviewer can independently verify
- **No undefined behavior** — run Miri (`cargo miri test`) on any code that touches raw pointers, FFI, or unions
- **No silent panics** — use `Result` for fallible operations; document `unwrap()` calls with a reason

---

## 📏 Rust Idioms

### 1. Types Over Comments

Let the type system express invariants the compiler can enforce:

```rust
// ❌ Bad: magic constants and runtime assertions
fn process(val: f32) {
    assert!(val >= 0.0 && val <= 1.0);
    // ...
}

// ✅ Good: type enforces the invariant at compile time
struct Normalized(f32);

impl Normalized {
    pub fn new(val: f32) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&val) {
            return Err(format!("{val} is not in [0, 1]"));
        }
        Ok(Self(val))
    }
}
```

### 2. Enums Over Booleans

```rust
// ❌ Bad: boolean blindness
fn alert(urgent: bool, severity: u8) { /* ... */ }

// ✅ Good: readable, exhaustive, compile-time checked
enum Severity { Low, Moderate, High, Critical }
enum AlertKind { Haptic, Led, Log, None }

fn alert(severity: Severity, kind: AlertKind) { /* ... */ }
```

### 3. Match Over If-Else Chains

```rust
// ❌ Bad: if-else cascade
fn severity_label(s: f32) -> &'static str {
    if s >= 0.8 { "Critical" }
    else if s >= 0.6 { "High" }
    else if s >= 0.3 { "Moderate" }
    else { "Low" }
}

// ✅ Good: exhaustive match, compiler warns on new variants
fn severity_label(severity: Severity) -> &'static str {
    match severity {
        Severity::Critical => "Critical",
        Severity::High => "High",
        Severity::Moderate => "Moderate",
        Severity::Low => "Low",
    }
}
```

### 4. Iterators Over Loops

```rust
// ❌ Bad: imperative loop with mutable accumulator
let mut max_conf = 0.0;
for d in &detections {
    if d.confidence > max_conf {
        max_conf = d.confidence;
    }
}

// ✅ Good: functional, self-documenting
let max_conf = detections.iter()
    .map(|d| d.confidence)
    .fold(0.0, f32::max);
```

### 5. Traits for Dependency Inversion

```rust
// ✅ Good: trait abstracts the LLM backend
pub trait LlmInference {
    fn infer(&self, prompt: &str) -> Result<String, String>;
}

// Swap implementations without changing callers
struct MistralClient;
struct OpenAiClient;
impl LlmInference for MistralClient { /* ... */ }
impl LlmInference for OpenAiClient { /* ... */ }
```

---

## 🧹 Clean Code Principles

### Naming

| Principle | ❌ Bad | ✅ Good |
|-----------|--------|---------|
| Pronounceable | `fn proc_au_data(d: &[f32])` | `fn extract_action_units(features: &[f32])` |
| Searchable | `let t = 0.5` | `let confidence_threshold = 0.5` |
| No Hungarian | `f32_hrv_val` | `hrv_ms` |
| No abbreviations | `fn calc_incng()` | `fn detect_expression_incongruence()` |

### Function Size

A function should do **one thing** and fit on one screen (~40 lines). If you need a comment to explain a block, extract that block into a named function.

```rust
// ❌ Bad: does three things
fn process_frame(frame: &[f32]) -> f32 { /* 80 lines */ }

// ✅ Good: one function per concern
fn detect_face(frame: &[f32]) -> Result<FaceLandmarks, String> { /* ... */ }
fn score_aus(landmarks: &FaceLandmarks) -> ActionUnitProfile { /* ... */ }
fn compute_risk(au: &ActionUnitProfile) -> f32 { /* ... */ }
```

### No Dead Code

- Every `pub` item must have a use
- No commented-out code (that's what git history is for)
- No `#[allow(dead_code)]` without a reason — use it only for stubs during active development, remove before PR

---

## ⚡ Performance

### Zero-Cost Abstractions

Prefer abstractions that compile away to nothing. If a trait or closure adds runtime overhead, document the trade-off.

```rust
// ✅ Good: monomorphized, zero-cost
fn process<T: AsRef<[f32]>>(data: T) { /* compiled to direct memory access */ }

// ❌ Bad: unnecessary heap allocation
fn process(data: &[f32]) {
    let vec = data.to_vec(); // Why? You already have a slice
}
```

### Allocation Discipline

- No allocations in the hot path (inference loop)
- Prefer `ArrayVec` or fixed-size arrays where the maximum size is known
- Use `&[T]` over `&Vec<T>` for function parameters

### Profile Before Optimizing

```rust
// ❌ Bad: guessing about bottlenecks
fn micro_optimization() { /* premature */ }

// ✅ Good: data-driven optimization
// Run: cargo bench or perf record
// Then optimize the hot path
```

---

## 🧪 Formal Verification & Testing

We treat correctness as a **compile-time property** wherever possible.

### Property-Based Tests

Use `proptest` or `quickcheck` to verify invariants:

```rust
/// Invariant: distance must decrease as pixel_width increases
proptest! {
    #[test]
    fn distance_inversely_proportional_to_pixel_width(
        pixel_width in 10..1000u32,
        real_width in 1.0..5.0f32,
    ) {
        let d1 = estimate_distance(pixel_width as f32, real_width, 650.0);
        let d2 = estimate_distance((pixel_width * 2) as f32, real_width, 650.0);
        prop_assert!(d2 < d1); // Larger bbox → closer → smaller distance? No wait...
        // This catches logic errors in the formula!
    }
}
```

### Panic-Free Guarantees

All public API functions must be panic-free unless documented otherwise. Use:

- `.ok()?` over `.unwrap()`
- `.get(index)` over `[index]`
- `checked_add()` / `checked_mul()` over `+` / `*`
- `saturating_sub()` where underflow is semantically valid

### Invariant Documentation

Every `struct` with internal invariants must document them:

```rust
/// A normalized score in [0, 1].
///
/// INVARIANT: `inner` must always be between 0.0 and 1.0 inclusive.
/// This is enforced at construction. Any other method that constructs
/// directly must also uphold this.
pub struct NormalizedScore {
    inner: f32,
}
```

---

## 🚫 PR Rejection Criteria

Your PR **will be rejected** if it contains any of the following:

| # | Violation | Example |
|---|-----------|---------|
| 1 | **Unsafe without SAFETY comment** | `unsafe { ... }` with no `// SAFETY:` |
| 2 | **Dead code** | Commented-out code, unused imports, `#[allow(dead_code)]` without reason |
| 3 | **Silent unwrap in production path** | `.unwrap()` or `.expect()` with no explanation |
| 4 | **Boolean blindness** | `fn alert(urgent: bool, show_led: bool)` instead of an enum |
| 5 | **Magic numbers** | `if score > 0.8 { ... }` with no named constant |
| 6 | **Function > 50 lines** | Without justification in a doc comment |
| 7 | **No tests for new logic** | Any non-trivial function without a `#[test]` |
| 8 | **Premature allocation on hot path** | `to_vec()`, `clone()`, `format!()` in the inference loop |
| 9 | **Leaking unsafe abstraction** | Wrapping unsafe code in a safe function without upholding safety invariants |
| 10 | **Violating the Golden Rule** | Any code that profiles the *other person* instead of the user |

### Pre-Submission Checklist

Before opening a PR, verify:

- [ ] `cargo test` passes (all tests, including doc-tests)
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo fmt` has been run
- [ ] No `todo!()` or `unimplemented!()` remain in production code
- [ ] All new public items have doc comments
- [ ] `// SAFETY:` comments on every `unsafe` block
- [ ] No dead code or commented-out code
- [ ] Property-based tests added for critical math/logic

---

## 🔗 References

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Clean Code (Martin)](https://www.oreilly.com/library/view/clean-code/9780136083238/)
- [Rustonomicon — Unsafe Code Guidelines](https://doc.rust-lang.org/nomicon/)
- [Zero-Cost Abstractions in Rust](https://blog.rust-lang.org/2015/05/11/traits.html)

---

<div align="center">

*"A program is never wrong in the way you expect. Formalize your assumptions, verify your invariants, and never trust a runtime assertion you could have made a compile-time guarantee."*

</div>
