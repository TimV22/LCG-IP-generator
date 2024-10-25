use std::collections::HashSet;
use std::time::Instant;
use rand::Rng;

// Constants for the Linear Congruential Generator
// https://en.wikipedia.org/wiki/Linear_congruential_generator#Parameters_in_common_use
const MODULUS: u32 = u32::MAX;
const MULTIPLIER: u32 = 1664525;
const INCREMENT: u32 = 1013904223;

struct IPGenerator {
    seed: u32,
}

impl IPGenerator {
    fn new(seed: u32) -> Self {
        IPGenerator { seed }
    }
}

impl Iterator for IPGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        // Apply the LCG formula to generate the next pseudo-random number
        self.seed = (MULTIPLIER.wrapping_mul(self.seed).wrapping_add(INCREMENT)) % MODULUS;

        let ip = format!(
            "{}.{}.{}.{}",
            (self.seed >> 24) & 0xFF,
            (self.seed >> 16) & 0xFF,
            (self.seed >> 8) & 0xFF,
            self.seed & 0xFF
        );

        Some(ip)
    }
}

fn main() {
    generate_ip_with_time(2);
    // generate_ip_with_time_and_check_dupes(100000000);
}

fn generate_ip_with_time(num_ips: u32) {
    let mut ip_generator = IPGenerator::new();

    let start = Instant::now();

    for _ in 0..num_ips {
        ip_generator.next();
    }

    let duration = start.elapsed();

    println!("Time elapsed: {:?}", duration);
}

fn generate_ip_with_time_and_check_dupes(num_ips: u32) {
    let mut ip_generator = IPGenerator::new();
    let mut seen_ips = HashSet::new();
    let mut duplicate_count = 0;

    let start = Instant::now();

    for i in 0..num_ips {
        if i % 1000000 == 0 {
            println!("{} ", i);
        }
        if let Some(ip) = ip_generator.next() {
            if !seen_ips.insert(ip.clone()) {
                println!("Duplicate IP: {}", ip);
                duplicate_count += 1;
            }
        }
    }

    let duration = start.elapsed();

    println!("{} IPs created", seen_ips.len());
    println!("Number of duplicate IPs: {}", duplicate_count);
    println!("Time elapsed: {:?}", duration);
}