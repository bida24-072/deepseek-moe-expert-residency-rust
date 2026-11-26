// =====================================================================
// Project: MoE Expert Residency Manager
// Author:  Theo Anold Gaopalelwe
// Context: Mitigating the low-VRAM MoE inference bottleneck discussed
//          in deepseek-ai/Engram Issue #8.
// =====================================================================

mod policy;
mod simulator;

use simulator::run_benchmark;

fn main() {
    println!("--- MoE Residency Engine ---");
    println!("Status: Booting allocation policy...\n");

    // 1. Hardware Constraints
    let total_vram_gb: f64 = 12.0;
    let kv_cache_reserved_gb: f64 = 4.0;
    let vram_limit_gb: f64 = total_vram_gb - kv_cache_reserved_gb; // 8.0 GB

    // 2. Model Architecture Limits (Simulating Qwen3-30B-A3B)
    let total_experts: usize = 64;
    let expert_size_gb: f64 = 0.25; 

    println!("=== Running Benchmark (1,000 Tokens) ===");
    let result = run_benchmark(vram_limit_gb, total_experts, expert_size_gb);

    // 3. Output Results
    println!("Total VRAM:       {:.2} GB", total_vram_gb);
    println!("KV Cache Reserve: {:.2} GB", kv_cache_reserved_gb);
    println!("Expert Budget:    {:.2} GB\n", vram_limit_gb);

    println!("FIFO Cache Misses (SSD Hits):      {}", result.fifo_misses);
    println!("Frequency Cache Misses (SSD Hits): {}", result.freq_misses);
    
    println!("\nLatency Improvement: {:.2}%", result.latency_improvement);
}