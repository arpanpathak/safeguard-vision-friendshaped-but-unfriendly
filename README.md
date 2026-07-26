<div align="center">

### 🌲 THE THREE GUARDIANS OF THE CASCADES 🌲

<br>

<img src="art/three-guardians.svg" alt="Three Guardians of the Cascades — Grizzly, Black Bear, Polar Bear" width="100%">

<br><br>

| Guardian | Spirit | Famous Saying | What It Means Here |
|----------|--------|--------------|-------------------|
| 🐻 **Black Bear** | *Protector of the understory* | **"If it's black, don't hurt"** | The quiet observer. Read the subtle signals. Know when to walk away. |
| 🐻‍❄️ **Grizzly** | *Guardian of the high country* | **"If it's grizzly, bear spray please. Thank you GTFO!"** | When danger is real — loud, clear, actionable alerts. No ambiguity. |
| 🐻‍❄️ **Polar Bear** | *Warden of the frozen edge* | **"If it's polar, you won't see another year of solar"** | Some situations are so toxic you must cut all contact. The emergency wipe. |

> *"The black bear knows every berry patch. The grizzly knows every trail. The polar bear knows no mercy."*

</div>

# 😎 Safeguard Vision — Friendshaped But Unfriendly

> *"Because your heart deserves a second opinion."*
> *— Standing guard over your perception, like the bears of the Cascades.*

**Open‑source AI‑powered smart glasses that detect love bombing and emotional manipulation — in real time.**

[![License: AGPL v3](https://img.shields.io/badge/License-AGPLv3-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.85+-orange.svg)](https://www.rust-lang.org)
[![CUDA-Oxide](https://img.shields.io/badge/GPU-CUDA--Oxide-76B900)](https://github.com/NVIDIA/cuda-oxide)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

</div>

---

## 🧠 What Is This?

**Safeguard Vision** is a wearable AI system that helps **trauma survivors, abuse recovery individuals, and anyone who's been gaslit** regain trust in their own perception.

It runs on smart glasses + a companion wrist band, processing:

| Input | What We Look For |
|-------|-----------------|
| 🗣️ Speech | Excessive flattery, future faking, guilt‑tripping |
| 😮 Face | Micro‑expressions incongruent with words |
| 💓 Body | Stress spikes, breath shifts, tension |

### 🌟 The Golden Rule

> **Never profile the other person. Only profile the user's response to them.**

This isn't about judging others — it's about giving the user data about their own nervous system.

---

## 🏗️ Architecture

```
┌──────────────────────────────────────────────────────────┐
│                   SAFEGUARD VISION                       │
├──────────────────────────────────────────────────────────┤
│  [Smart Glasses]                                         │
│    ├─ 📷 Camera (120fps, IR)                             │
│    ├─ 🎤 Microphone Array                                │
│    ├─ 💡 LED Indicator (privacy)                         │
│    ├─ 📳 Haptic Motor (discrete vibration)               │
│    └─ ⚡ Edge AI (Jetson Orin Nano)                      │
│         ↓ (Bluetooth / USB-C)                            │
│  [Companion Band]                                        │
│    ├─ ❤️ HRV                                            │
│    ├─ 💧 Galvanic Skin Response                          │
│    ├─ 🌡️ Skin Temperature                                │
│    └─ 💉 Blood Volume Pulse                              │
│         ↓                                                │
│  [Mobile App (React PWA)] ←→ [☁️ Cloud (optional)]       │
└──────────────────────────────────────────────────────────┘
```

### Data Pipeline

```
[Raw Inputs] → [Feature Extraction] → [Risk Scoring] → [Alert]
     │                │                     │              │
     ├─ 📷 Video      ├─ 👤 Landmarks/AU    ├─ ⚖️ Fusion   ├─ 📳 Haptic
     ├─ 🎤 Audio      ├─ 📝 Text Patterns   ├─ 🎯 Threshold ├─ 💡 LED
     └─ 💓 Physiology └─ 📊 Stress Metrics   └─ 📋 Severity  └─ 📋 Log
```

---

## 🚧 Project Status

**⚠️ Pre‑alpha / Skeleton.** This is the initial scaffold with `todo!()` stubs.

- [x] Project structure & module layout
- [x] CUDA‑Oxide kernel stubs
- [x] Vision pipeline skeleton (landmarks, AUs, incongruence)
- [x] NLP pipeline skeleton (transcription, pattern detection)
- [x] Physiology skeleton (HRV/GSR scoring)
- [x] Risk fusion engine skeleton
- [ ] **Real GPU kernel implementation** ![WIP](https://img.shields.io/badge/-TODO-red)
- [ ] **Whisper.cpp integration** ![WIP](https://img.shields.io/badge/-TODO-red)
- [ ] **Mistral 7B local LLM integration** ![WIP](https://img.shields.io/badge/-TODO-red)
- [ ] **Companion band BLE driver** ![WIP](https://img.shields.io/badge/-TODO-red)
- [ ] **Real‑time pipeline wiring** ![WIP](https://img.shields.io/badge/-TODO-red)

See the full roadmap in [CONTRIBUTING.md](CONTRIBUTING.md).

---

## 🛠️ Tech Stack

| Layer | Technology |
|-------|-----------|
| 👓 Glasses HW | Rokid Max Pro / Meta Ray‑Ban |
| ⚡ Edge AI | NVIDIA Jetson Orin Nano (8GB) |
| 👁️ Vision AI | YOLOv8, Mediapipe, ArcFace |
| 🗣️ NLP | Whisper (STT), Mistral 7B (local LLM) |
| 💓 Physiology | HRV, GSR, skin temp, BVP sensors |
| 📱 Mobile | React PWA |
| 🖥️ Backend | Python/Flask + ONNXRuntime |
| 🦀 GPU Kernels | CUDA‑Oxide (Rust → PTX) |

---

## 🚀 Quick Start (Development)

```bash
# Prerequisites: Rust 1.85+, CUDA‑Oxide toolchain
git clone https://github.com/arpanpathak/safeguard-vision-friendshaped-but-unfriendly.git
cd safeguard-vision-friendshaped-but-unfriendly

# Build (will fail on todo!() stubs — expected!)
cargo build
```

> **Note:** `cargo build` will fail on `todo!()` macros until real implementations replace the stubs. This is intentional — pick a module and start hacking!

---

## 🔒 Privacy & Ethics (Non‑Negotiable)

| Principle | Implementation |
|-----------|---------------|
| 🚫 Never profile the other person | Only analyze the user's response |
| 💻 On‑device processing | No cloud by default |
| 🚫 No recording | Real‑time inference only, no storage |
| 🔐 User owns data | Exportable and deletable anytime |
| 🕵️ Decoy mode | For user safety in abusive situations |
| 🔑 Biometric lock | Only the user can access |
| 💣 Emergency wipe | One‑touch data destruction |

---

## 📜 License

**GNU AGPL v3** — because your work shouldn't be swallowed by a corporation's proprietary vault.

- ✅ **You can** use, fork, modify, and distribute — even commercially.
- ✅ **You can** deploy it as a service.
- ❌ **You cannot** incorporate it into a closed‑source proprietary product without releasing your source code.
- ❌ **Evil tech corporations** cannot steal this and lock it behind a paywall.

See the [full license](LICENSE) for details.

---

## 🤝 Contributing

We welcome contributions with open arms (but not love bombing)!

- 🐛 Found a bug? Open an issue
- 💡 Have an idea? Start a discussion
- 🔧 Want to code? Read [CONTRIBUTING.md](CONTRIBUTING.md)

---

<div align="center">

*"Is what I am feeling right now mine, or is it an echo of what was done to me?"*

**— Safeguard Vision**

⭐ Star this repo if you believe perception deserves protection.

</div>
