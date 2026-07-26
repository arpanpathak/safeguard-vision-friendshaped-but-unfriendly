# ☁️ Cloud GPU Training Guide

> *"I need an NVIDIA GPU to train this, but I don't have one. What do I do?"*

You don't need to drop $2,000+ on a GPU. Rent one by the hour for pocket change. This guide covers the cheapest options and exact setup steps.

---

## 🏆 Cheapest Cloud GPU Providers (2026)

For most ML/AI work, a **single RTX 3090 (24GB)** is more than enough. You don't need A100s or H100s.

| Provider | GPU | VRAM | Hourly Cost | Best For | Catch |
|----------|-----|------|-------------|----------|-------|
| **Vast.ai** 🥇 | RTX 3090 | 24GB | **~$0.15–0.22/hr** | Absolute cheapest | Peer-to-peer — check reviews before renting |
| **RunPod** 🥇 | RTX 3090 | 24GB | **~$0.19/hr** | Best price + reliability combo | Community Cloud tier (not dedicated) |
| **RunPod** | RTX 4090 | 24GB | ~$0.34/hr | Faster training | Worth it if you train frequently |
| **AutoDL** | RTX 3090 | 24GB | ~¥1.50/hr (~$0.20) | Cheapest in Asia | Chinese UI, Alipay required |
| **Lambda Labs** | RTX 4090 | 24GB | ~$0.35/hr | Reliable, good support | Slightly pricier |
| **Google Colab Pro** | T4 | 16GB | **$10/month** | Prototyping | 8-hour session limit |
| **Google Colab Pro+** | A100 | 40GB | $50/month | Heavy training | Still time-limited |
| **Paperspace** | RTX 4000 Ada | 16GB | ~$0.23/hr | Good UI, easy setup | Less VRAM than 3090 |

---

## 🚀 Quick Start: RunPod (Recommended)

### Step 1: Sign Up

1. Go to [runpod.io](https://runpod.io)
2. Sign up (GitHub account works)
3. Add $10 in credits

### Step 2: Launch a GPU Instance

1. Click **"Pod"** → **"Community Cloud"**
2. Filter: GPU = `RTX 3090`, Disk ≥ `50 GB`
3. Pick an instance with `~$0.19/hr` pricing
4. Template: **"RunPod PyTorch 2.x"** (CUDA + torch pre-installed)
5. Click **"Deploy On-Demand"**

### Step 3: Setup CUDA-Oxide (Rust GPU Kernels)

For Safeguard Vision's GPU kernels, you need the CUDA-Oxide toolchain:

```bash
# SSH into the instance
ssh -p <PORT> root@<IP_ADDRESS>

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify CUDA is available
nvidia-smi
nvcc --version

# Clone and build
git clone https://github.com/arpanpathak/safeguard-vision-friendshaped-but-unfriendly.git
cd safeguard-vision-friendshaped-but-unfriendly

# Build (will fail on todo!() stubs — expected until real kernel code is written)
cargo build
```

### Step 4: Fine-Tune Mistral 7B (Local LLM)

If you need to fine-tune the Mistral 7B model for love bombing pattern detection:

```bash
# Install llama.cpp with CUDA support
git clone https://github.com/ggerganov/llama.cpp
cd llama.cpp
make LLAMA_CUDA=1

# Download Mistral 7B (quantized)
wget -O models/mistral-7b-instruct-v0.2.Q4_K_M.gguf \
    https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF/resolve/main/mistral-7b-instruct-v0.2.Q4_K_M.gguf

# Run inference test
./main -m models/mistral-7b-instruct-v0.2.Q4_K_M.gguf \
    -p "Analyze this for love bombing patterns: 'You are my soulmate. We will be together forever.'" \
    -n 200
```

> **Note:** 24GB VRAM is enough for 7B parameter models with 4-bit quantization. For full fine-tuning (not just inference), you'll want a **RunPod RTX 4090 ($0.34/hr)** or **Colab Pro+ A100**.

### Step 5: Download Your Results

```bash
# Compress and download
tar -czf safeguard-results.tar.gz models/ runs/

# Download via SCP or use RunPod's web file browser
```

---

## ⚡ Alternative: Vast.ai (Cheapest)

```bash
# 1. Browse: https://vast.ai/ → search "RTX 3090"
# 2. Filter: rentable = yes, verified = yes
# 3. Pick an instance ~$0.15-0.18/hr
# 4. Launch with PyTorch template
# 5. SSH in and follow same setup steps
```

### ⚠️ Vast.ai Pro Tips

- **Check "Verified" hosts only** — avoids bad actors
- **Look for "Docker: pytorch/pytorch"** — saves setup time
- **Use `tmux`** — so training doesn't die if SSH disconnects

```bash
tmux new-session -s training
# ... run commands ...
# Ctrl+B then D to detach
# tmux attach -t training to reattach
```

---

## 🧪 Option: Google Colab (For Quick Experiments)

Best for prototyping Mistral or Whisper model experiments.

| Plan | Price | GPU | Limit |
|------|-------|-----|-------|
| Free | $0 | T4 (16GB) | 2-hour sessions |
| Pro | $10/mo | T4/V100 | 8-hour sessions |
| Pro+ | $50/mo | A100 (40GB) | 24-hour sessions |

---

## 📊 GPU Needs by Module

| Module | GPU Requirement | Cheapest Cloud Option |
|--------|----------------|----------------------|
| **CUDA-Oxide kernels** (`src/kernels/`) | Any NVIDIA GPU with CUDA | **RunPod RTX 3090** — $0.19/hr |
| **Mistral 7B inference** (`src/nlp/patterns.rs`) | 8GB+ VRAM (4-bit quantized) | **Colab Pro T4** — $10/mo |
| **Mistral 7B fine-tuning** | 24GB+ VRAM (LoRA) | **RunPod RTX 4090** — $0.34/hr |
| **Whisper STT fine-tuning** (`src/nlp/transcript.rs`) | 16GB+ VRAM | **Vast.ai RTX 3090** — $0.15/hr |
| **YOLOv8 face detection** | 8GB+ VRAM | **RunPod RTX 3090** — $0.19/hr |

---

## 🧠 Pro Tips

### Save Money

| Tip | Saves |
|-----|-------|
| Use **spot/community** instances, not on-demand | 60–70% |
| **Stop instances when idle** — set auto-stop timers | 100% of idle cost |
| **Mount network storage** (S3) — keep data between sessions | No re-upload costs |
| **Use `tmux`** — detach without killing work | Prevents wasted runs |

### Avoid Pitfalls

- **Don't use A100 for kernel dev** — overkill. RTX 3090 is the sweet spot
- **Don't train on spot without checkpointing** — they can terminate anytime
- **Do enable auto-stop** — RunPod lets you set "stop after X minutes idle"

---

## 🔗 Quick Links

| Resource | URL |
|----------|-----|
| RunPod | https://runpod.io |
| Vast.ai | https://vast.ai |
| Lambda Labs | https://lambdalabs.com |
| Google Colab | https://colab.research.google.com |
| GPU Price Comparison | https://gpus.io |

---

<div align="center">

*You don't need a $3,000 GPU. You need $0.19 an hour and this guide.*

⭐ **Star the project if this helped you get started.**

</div>
