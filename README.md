Theo，下面是你要的 README 草稿。我按照你的要求，把问题陈述、Engram Issue #8、12GB 笔记本实测数据、瓶颈分析、你的范围选择，以及你的个人背景都写进去了。语气保持真实、不夸大——这恰恰是 DeepSeek 工程师会尊重的东西。

---

# KV-Cache & Expert Budget Manager for Low-VRAM MoE Inference (Rust Prototype)

## The Problem

DeepSeek's models are powerful, but they are heavy. Most ML practitioners and students do not have access to the multi-GPU clusters that DeepSeek's own infrastructure assumes. This is not a complaint—it is a documented gap.

**Engram Issue #8** on the official `deepseek-ai/Engram` repository states the problem precisely: the Rust ML ecosystem lacks structured support for attention/KV/memory dependencies, **especially for people trying to work with large MoEs like DeepSeek on somewhat limited or partially offloaded hardware** . The issue author notes that DeepSeek's models "require a ton of VRAM in conventional use—not the sort of kit to which most ML practitioners or students have access" .

This project is a direct response to that gap. It is not a complete inference engine. It is not a generic offloading tool. It is a **targeted Rust prototype** that explores one narrow question:

> Given a fixed VRAM budget, how should a low-VRAM system decide which MoE experts stay resident on the GPU and which get offloaded—and how should the KV cache be managed alongside that expert budget?

## What the Data Says (12GB VRAM Reality)

A real-world test on an **RTX 5070 Ti Laptop GPU with 12GB VRAM** (32GB system RAM, NVMe SSD) running DeepSeek V4 Flash via CUDA + SSD streaming produced the following results :

| Configuration | Prefill Speed | Generation Speed | Planned GPU Memory |
|---|---|---|---|
| 10GB expert budget | 1.74 t/s | 0.65 t/s | 11.41 GiB |
| 8GB expert budget | 1.95 t/s | **0.73 t/s** | 9.40 GiB |

The critical finding: **a smaller expert budget (8GB) was slightly faster than a larger one (10GB)**, while also leaving substantially more VRAM headroom . The test author's interpretation: "On a very constrained GPU, maximizing cache residency is not necessarily the fastest configuration. The additional memory pressure near the VRAM limit may outweigh the benefit of keeping more experts resident" .

### The Bottleneck

The 12GB test reveals two distinct bottlenecks:

1. **MMQ prefill OOM**: The default MMQ prefill path exceeded available VRAM and crashed until it was disabled (`DS4_CUDA_MMQ=0`) . This is a memory allocation problem during the prefill phase, not the decode phase.
2. **Cache budget trade-off**: More resident experts ≠ faster inference on constrained hardware. The system needs a **smarter policy** for deciding what stays on GPU, not just a larger cache .

## My Scope (Deliberately Narrow)

I am **not** building a complete inference engine. I do not have the resources, the hardware, or the team for that—and pretending otherwise would be dishonest.

I am building a **Rust prototype** that addresses one specific question from the Engram Issue #8 discussion:

> **Given a limited VRAM budget, how do we decide which experts stay resident on the GPU and which get offloaded, and how do we manage the KV + expert cache budget together on low-VRAM devices?**

The prototype will:
- Model the VRAM budget as a constraint
- Implement a configurable policy for expert residency (e.g., LRU-based, frequency-based, or routing-aware)
- Track KV cache usage alongside expert cache usage
- Benchmark different policies against a small model or synthetic workload
- Document what works, what fails, and what the 12GB test data suggests about policy design

The prototype will **not**:
- Support full DeepSeek V4 inference
- Compete with vLLM or SGLang
- Claim to be production-ready

## Tools

- **Rust** (`cargo`, `clippy`, `rustfmt`) — the language DeepSeek uses for its DSec infrastructure
- **GitHub** — public repo, versioned commits, issue tracking
- **VS Code** — editor

## Why This Matters (To Me)

I am a third-year Business Intelligence and Data Analytics student at Botswana School of Business Sciences. I do not have a research lab, a GPU cluster, or a competition gold medal. What I have is my sister's Acer laptop, a GitHub account, and a Rust compiler.

I am building this because I want to work at DeepSeek—the platform that shaped how I think about AI—and I want to contribute to areas where I can see real gaps. I am not applying with a polished CV. I am applying with a prototype that addresses a problem DeepSeek's own community has already acknowledged.

If this prototype is useful, I will have learned something real. If it fails, I will have learned something real. Either way, the work is public.

## Status

**Early prototype. Not yet functional.** This README will be updated as the first module is built.

## References

- Engram Issue #8: [github.com/deepseek-ai/Engram/issues/8](https://github.com/deepseek-ai/Engram/issues/8) 
- 12GB VRAM test: [github.com/JustVugg/colibri/discussions/963](https://github.com/JustVugg/colibri/discussions/963) 

---

**Then update your portfolio's PulaTrack card or add a new project card** that links to this repo once you push it. The README above is your proof. The code comes next.

When you're ready to start the first Rust module, tell me and I'll write the `Cargo.toml` and the first file structure—then you run it, paste the errors, and we iterate.
