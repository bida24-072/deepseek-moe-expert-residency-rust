# MoE Expert Residency Manager (Rust)

> **Status:** Early Prototype · **Target:** Low-VRAM MoE Inference  
> **Context:** Addressing the memory bottleneck discussed in `deepseek-ai/Engram` Issue #8.

## The Problem
DeepSeek's models (and large MoEs like Qwen3-30B-A3B) are powerful, but they are incredibly heavy. The Rust ML ecosystem currently lacks structured tooling for managing memory dependencies when these models are constrained to consumer hardware. 

When VRAM is limited, the entire model cannot fit into memory. Experts must be offloaded to NVMe/SSD. The bottleneck is no longer the GPU matrix math—it is the latency penalty of swapping expert weights in and out of VRAM.

## The Evidence (12GB VRAM Reality)
Real-world testing on a 12GB VRAM laptop running SSD-streamed MoE inference revealed a counter-intuitive bottleneck:

| Configuration | Prefill Speed | Generation Speed | Planned GPU Memory |
|---|---|---|---|
| 10GB expert budget | 1.74 t/s | 0.65 t/s | 11.41 GiB |
| 8GB expert budget | 1.95 t/s | **0.73 t/s** | 9.40 GiB |

**The Finding:** A smaller expert budget (8GB) outperformed a larger one (10GB). On constrained hardware, maximizing cache residency is not the fastest configuration—the memory pressure near the VRAM limit outweighs the benefit. 

The system needs a **smarter policy** for deciding what stays resident, not just a larger cache.

## The Solution (Deliberately Narrow Scope)
This is not a full inference engine. It is a targeted Rust prototype that answers one specific question: 

> *Given a limited VRAM budget, how do we decide which experts stay resident on the GPU and which get offloaded to minimize latency?*

The prototype models the 12GB VRAM constraint (reserving 4GB for KV Cache, leaving an 8GB expert budget) and simulates token routing. It implements a **Frequency-Based Residency Policy**:
1. Tracks the routing frequency of each expert.
2. Permanently pins the "hottest" experts into the fast VRAM budget.
3. Dynamically streams "cold" experts using an LRU (Least Recently Used) cache.

## Benchmark Results (Synthetic Workload)

Simulating 1,000 token routing requests with an 8GB expert budget (12GB VRAM - 4GB KV Cache reserve):

- **Naive FIFO Policy:** 123 Cache Misses (SSD Hits)
- **Frequency-Based Policy:** 81 Cache Misses (SSD Hits)
- **Result:** **34.15% reduction** in latency-inducing SSD fetches.

## Architecture
- `src/main.rs` — Hardware constraints, synthetic token routing simulation, and benchmark output.
- `src/policy.rs` — Residency algorithms: `simulate_fifo` (naive) vs. `simulate_frequency` (smart).
- **Dependencies:** `rand` crate for accurate token distribution simulation.

## Why Rust?
DeepSeek builds core infrastructure (like DSec) in Rust. Systems-level memory management requires strict safety and zero-cost abstractions. This prototype proves residency logic can be handled natively without Python or C++ overhead.

## Why This Matters (To Me)
I am a third-year Business Intelligence and Data Analytics student at Botswana School of Business Sciences. I do not have a research lab or a GPU cluster. I have a laptop, a Rust compiler, and a GitHub account. 

I am building this because I want to work at DeepSeek—the platform that shaped how I think about AI—and I want to contribute to areas where I can see real gaps. I am not applying with a polished CV. I am applying with a prototype that addresses a problem DeepSeek's own community has already acknowledged. If it works, I learned something real. If it fails, I learned something real. 

## Roadmap
- [x] Define problem scope and 12GB hardware constraints.
- [x] Benchmark FIFO vs. Frequency-based eviction (Cache Miss counts).
- [x] Implement LRU (Least Recently Used) cache for cold experts.
- [ ] Integrate real routing traces from a small MoE model.
- [ ] Add KV Cache eviction logic alongside expert residency.

## References
- Engram Issue #8: `github.com/deepseek-ai/Engram/issues/8`
- 12GB VRAM Test Data: `github.com/JustVugg/colibri/discussions/963`

---
**Author:** Theo Anold Gaopalelwe  
*Founder, AXIOMIQ Digital Innovations*  
[GitHub Profile](https://github.com/bida24-072)
