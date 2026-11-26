// src/simulator.rs
// Engine for generating synthetic workloads and benchmarking memory policies.

use crate::policy::{Expert, simulate_fifo, simulate_frequency};
use rand::Rng;

pub struct BenchmarkResult {
    pub fifo_misses: u32,
    pub freq_misses: u32,
    pub latency_improvement: f64,
}

pub fn run_benchmark(vram_budget_gb: f64, total_experts: usize, expert_size_gb: f64) -> BenchmarkResult {
    // 1. Initialize experts with skewed distribution (hot vs cold)
    let mut experts: Vec<Expert> = Vec::new();
    for i in 0..total_experts {
        let frequency = if i < 8 { 5000 } else if i < 24 { 1000 } else { 50 };
        experts.push(Expert {
            id: i,
            size_gb: expert_size_gb,
            activation_count: frequency,
        });
    }

    // 2. Generate 1,000 token requests with realistic routing behavior
    let mut token_requests = Vec::new();
    let mut rng = rand::thread_rng();
    
    for _ in 0..1000 {
        let roll: f64 = rng.gen();
        let expert = if roll < 0.6 { 
            0 // 60% to Expert 0
        } else if roll < 0.8 { 
            1 // 20% to Expert 1
        } else { 
            rng.gen_range(2..total_experts) // 20% distributed across cold experts
        };
        token_requests.push(expert);
    }

    // 3. Run Policies
    let fifo_misses = simulate_fifo(&experts, vram_budget_gb, &token_requests);
    let freq_misses = simulate_frequency(&experts, vram_budget_gb, &token_requests);

    // 4. Calculate improvement safely
    let improvement = if fifo_misses > 0 {
        ((fifo_misses as f64 - freq_misses as f64) / fifo_misses as f64) * 100.0
    } else {
        0.0
    };

    BenchmarkResult {
        fifo_misses,
        freq_misses,
        latency_improvement: improvement,
    }
}