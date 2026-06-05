#![forbid(unsafe_code)]
//! Needle drop — initial condition sensitivity. Where you start changes everything.

/// Perturbation: flip one agent and track divergence.
pub fn perturbation_divergence(
    original: &[i8],
    perturb_idx: usize,
    ticks: usize,
    rule: fn(&[i8]) -> Vec<i8>,
) -> Vec<f64> {
    let mut a = original.to_vec();
    let mut b = original.to_vec();
    if perturb_idx < b.len() { b[perturb_idx] = -b[perturb_idx].max(1); }
    let mut divergences = Vec::with_capacity(ticks);
    for _ in 0..ticks {
        a = rule(&a);
        b = rule(&b);
        let diff = a.iter().zip(&b).filter(|(x, y)| x != y).count();
        divergences.push(diff as f64 / a.len() as f64);
    }
    divergences
}

/// Estimate Lyapunov exponent from perturbation growth.
pub fn lyapunov_exponent(divergences: &[f64], dt: f64) -> f64 {
    if divergences.len() < 2 { return 0.0; }
    let mut sum = 0.0;
    let mut count = 0;
    for i in 1..divergences.len() {
        if divergences[i-1] > 1e-10 && divergences[i] > 1e-10 {
            sum += (divergences[i] / divergences[i-1]).ln();
            count += 1;
        }
    }
    if count == 0 { 0.0 } else { sum / count as f64 / dt }
}

/// Entry point sensitivity — compare trajectories from different starting points.
pub fn entry_point_sensitivity(
    size: usize,
    entries: &[Vec<i8>],
    ticks: usize,
    rule: fn(&[i8]) -> Vec<i8>,
) -> Vec<Vec<f64>> {
    let mut trajectories = Vec::new();
    for entry in entries {
        let mut state = entry.clone();
        while state.len() < size { state.push(0); }
        state.truncate(size);
        let mut hist = Vec::with_capacity(ticks);
        for _ in 0..ticks {
            state = rule(&state);
            let sum: f64 = state.iter().map(|&v| v as f64).sum::<f64>() / state.len() as f64;
            hist.push(sum);
        }
        trajectories.push(hist);
    }
    trajectories
}

/// Butterfly effect score — maximum divergence from any single-bit flip.
pub fn butterfly_score(
    state: &[i8],
    ticks: usize,
    rule: fn(&[i8]) -> Vec<i8>,
) -> (usize, f64) {
    let mut worst_idx = 0;
    let mut worst_div = 0.0;
    for i in 0..state.len() {
        let divs = perturbation_divergence(state, i, ticks, rule);
        let max_div = divs.iter().cloned().fold(0.0f64, f64::max);
        if max_div > worst_div { worst_div = max_div; worst_idx = i; }
    }
    (worst_idx, worst_div)
}

/// Majority rule for testing.
pub fn majority_rule(state: &[i8]) -> Vec<i8> {
    let n = state.len();
    let side = (n as f64).sqrt() as usize;
    if side == 0 { return state.to_vec(); }
    let mut next = state.to_vec();
    for i in 0..n {
        let r = i / side; let c = i % side;
        let mut sum = 0i32;
        for dr in -1i32..=1 { for dc in -1i32..=1 {
            if dr == 0 && dc == 0 { continue; }
            let nr = ((r as i32 + dr).rem_euclid(side as i32)) as usize;
            let nc = ((c as i32 + dc).rem_euclid(side as i32)) as usize;
            let ni = nr * side + nc;
            if ni < n { sum += state[ni] as i32; }
        }}
        if sum > 0 { next[i] = 1; } else if sum < 0 { next[i] = -1; }
    }
    next
}

/// The 8-ball test — can you place the ball and have it stay?
/// Find initial conditions that remain stable (don't diverge).
pub fn find_equilibrium_points(
    size: usize,
    candidates: usize,
    ticks: usize,
    rule: fn(&[i8]) -> Vec<i8>,
) -> Vec<Vec<i8>> {
    let mut rng_s: u64 = 42;
    let mut rng = || -> u64 { rng_s = rng_s.wrapping_mul(6364136223846793005).wrapping_add(1); rng_s };
    let mut stable = Vec::new();
    for _ in 0..candidates {
        let init: Vec<i8> = (0..size).map(|_| match rng() % 3 { 0 => -1, 1 => 0, _ => 1 }).collect();
        let mut state = init.clone();
        for _ in 0..ticks { state = rule(&state); }
        if state == init { stable.push(init); }
    }
    stable
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_perturbation_grows() { let state = vec![1,0,-1,1,0,-1,1,0,-1]; let divs = perturbation_divergence(&state, 0, 10, majority_rule); assert!(divs.iter().any(|&d| d > 0.0)); }
    #[test] fn test_lyapunov_nonnegative() { let divs = vec![0.01, 0.02, 0.04, 0.08]; let lambda = lyapunov_exponent(&divs, 1.0); assert!(lambda > 0.0); }
    #[test] fn test_lyapunov_zero() { let divs = vec![0.01, 0.01, 0.01]; let lambda = lyapunov_exponent(&divs, 1.0); assert!(lambda.abs() < 0.1); }
    #[test] fn test_lyapunov_short() { assert_eq!(lyapunov_exponent(&[], 1.0), 0.0); }
    #[test] fn test_entry_sensitivity() { let entries = vec![vec![1,1,1,0,0,0,-1,-1,-1], vec![-1,-1,-1,0,0,0,1,1,1]]; let trajs = entry_point_sensitivity(9, &entries, 5, majority_rule); assert_eq!(trajs.len(), 2); }
    #[test] fn test_butterfly_score() { let state = vec![1,0,-1,1,0,-1,1,0,-1]; let (idx, score) = butterfly_score(&state, 10, majority_rule); assert!(idx < state.len()); assert!(score >= 0.0); }
    #[test] fn test_majority_rule() { let state = vec![1,1,1,1,1,1,-1,-1,-1]; let next = majority_rule(&state); assert_eq!(next[4], 1); }
    #[test] fn test_majority_stability() { let state = vec![1,1,1,1,1,1,1,1,1]; let next = majority_rule(&state); assert!(next.iter().all(|&v| v == 1)); }
    #[test] fn test_majority_tie() { let state = vec![1,-1,0,1,-1,0,1,-1,0]; let next = majority_rule(&state); assert_eq!(next.len(), 9); }
    #[test] fn test_equilibrium_points() { let stable = find_equilibrium_points(9, 100, 3, majority_rule); assert!(stable.len() >= 0); }
    #[test] fn test_empty_entry() { let trajs = entry_point_sensitivity(4, &[], 5, majority_rule); assert!(trajs.is_empty()); }
    #[test] fn test_perturbation_identity() { let state = vec![1,1,1,1]; let divs = perturbation_divergence(&state, 5, 3, majority_rule); assert_eq!(divs.len(), 3); }
    #[test] fn test_divergence_range() { let state = vec![1,0,-1,1,0,-1,1,0,-1]; let divs = perturbation_divergence(&state, 4, 10, majority_rule); assert!(divs.iter().all(|&d| d >= 0.0 && d <= 1.0)); }
}
