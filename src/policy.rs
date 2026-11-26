// src/policy.rs
// Memory management policies and benchmark logic for MoE inference.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Expert {
    pub id: usize,
    pub size_gb: f64,
    pub activation_count: u32,
}

// Baseline: First-Come, First-Serve (FIFO)
// Keeps experts in memory until VRAM is full, then evicts the oldest ones.
pub fn simulate_fifo(experts: &[Expert], vram_limit: f64, token_requests: &[usize]) -> u32 {
    let mut vram_cache: Vec<usize> = Vec::new();
    let mut cache_misses = 0;
    let mut current_vram = 0.0;
    
    let mut size_map = HashMap::new();
    for e in experts {
        size_map.insert(e.id, e.size_gb);
    }

    for &req_expert in token_requests {
        if !vram_cache.contains(&req_expert) {
            // CACHE MISS: Expert not in VRAM, must fetch from SSD.
            cache_misses += 1;
            
            let size_needed = size_map[&req_expert];
            // Evict oldest experts until we have room
            while current_vram + size_needed > vram_limit && !vram_cache.is_empty() {
                let evicted = vram_cache.remove(0);
                current_vram -= size_map[&evicted];
            }
            
            // Load new expert into VRAM
            vram_cache.push(req_expert);
            current_vram += size_needed;
        }
    }
    cache_misses
}

// Proposed: Frequency-Based Residency
// Permanently pins the hottest experts in VRAM, streams the cold ones.
pub fn simulate_frequency(experts: &[Expert], vram_limit: f64, token_requests: &[usize]) -> u32 {
    // Sort experts by activation count to find the hottest ones
    let mut sorted_experts = experts.to_vec();
    sorted_experts.sort_by(|a, b| b.activation_count.cmp(&a.activation_count));

    let mut permanent_vram = HashMap::new();
    let mut current_vram = 0.0;

    // Fill VRAM with the hottest experts
    for e in &sorted_experts {
        if current_vram + e.size_gb <= vram_limit {
            permanent_vram.insert(e.id, e.size_gb);
            current_vram += e.size_gb;
        } else {
            break;
        }
    }

    let mut cache_misses = 0;
    let mut dynamic_cache: Vec<usize> = Vec::new(); // For the cold experts

    for &req_expert in token_requests {
        if permanent_vram.contains_key(&req_expert) {
            continue; // HIT: Expert is permanently in VRAM
        }

        // MISS: Expert is cold, check dynamic cache
        if !dynamic_cache.contains(&req_expert) {
            cache_misses += 1;
            
            // Simple eviction for cold experts
            if dynamic_cache.len() > 4 {
                dynamic_cache.remove(0);
            }
            dynamic_cache.push(req_expert);
        }
    }
    cache_misses
}
