// =====================================================================
// Project: MoE Expert Residency Manager
// Author:  Theo Anold Gaopalelwe
// Context: Mitigating the low-VRAM MoE inference bottleneck discussed
//          in deepseek-ai/Engram Issue #8.
// =====================================================================

mod policy;
use policy::{Expert, simulate_fifo, simulate_frequency};

fn main() {
    println!("--- MoE Residency Engine ---");
    println!("Status: Booting allocation policy...\n");

    // 1. Hardware Constraints (Based on 12GB VRAM Test Data)
    let total_vram_gb: f64 = 12.0;
    let kv_cache_reserved_gb: f64 = 4.0;
    let vram_limit_gb: f64 = total_vram_gb - kv_cache_reserved_gb; // 8.0 GB

    // 2. Model Architecture Limits (Simulating Qwen3-30B-A3B)
    let total_experts: usize = 64;
    let expert_size_gb: f64 = 0.25; 

    // 3. Initialize experts
    let mut experts: Vec<Expert> = Vec::new();
    for i in 0..total_experts {
        // Skewed distribution: Expert 0-7 are heavily used
        let frequency = if i < 8 { 5000 } else if i < 24 { 1000 } else { 50 };
        experts.push(Expert {
            id: i,
            size_gb: expert_size_gb,
            activation_count: frequency,
        });
    }

    // 4. Generate Synthetic Token Routing Requests (1000 tokens)
    // We simulate a real workload where some experts are popular.
    let mut token_requests = Vec::new();
    for _ in 0..1000 {
        let expert = if rand_f64() < 0.6 { 
            0 // 60% of the time, it routes to Expert 0
        } else if rand_f64() < 0.8 { 
            1 // 20% of the time, Expert 1
        } else { 
            (rand_f64() * 64.0) as usize // Random cold expert
        };
        token_requests.push(expert);
    }

    // 5. Run Benchmarks
    println!("=== Running Benchmark (1,000 Tokens) ===");
    
    let fifo_misses = simulate_fifo(&experts, vram_limit_gb, &token_requests);
    let freq_misses = simulate_frequency(&experts, vram_limit_gb, &token_requests);

    // 6. Output Results
    println!("Total VRAM:       {:.2} GB", total_vram_gb);
    println!("KV Cache Reserve: {:.2} GB", kv_cache_reserved_gb);
    println!("Expert Budget:    {:.2} GB\n", vram_limit_gb);

    println!("FIFO Cache Misses (SSD Hits):      {}", fifo_misses);
    println!("Frequency Cache Misses (SSD Hits): {}", freq_misses);
    
    let improvement = ((fifo_misses as f64 - freq_misses as f64) / fifo_misses as f64) * 100.0;
    println!("\nLatency Improvement: {:.2}%", improvement);
}

// Simple pseudo-random generator for synthetic data
fn rand_f64() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    (nanos % 1000) as f64 / 1000.0
}
