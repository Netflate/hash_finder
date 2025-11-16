use sha2::{Digest, Sha256};
use std::env;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
    Arc,
};
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();

    //defaults
    let mut num_zeros: usize = 3;
    let mut results_needed: usize = 6; 
   
    // Arguments 
    for i in 0..args.len() {
        if args[i] == "-N" && i + 1 < args.len() {
            num_zeros = args[i + 1].parse().unwrap();
        }
        if args[i] == "-F" && i + 1 < args.len() {
            results_needed = args[i + 1].parse().unwrap();
        }
    }

    // The program uses as many threads as the system can provide, or uses 8 if it wasn't possible to determine

    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(8);

    /* 
        The logic of my code is based on the idea that each thread works with its own block of numbers.  
        The size of this block is defined by chunk_size (I chose 100 as a balance between speed and keeping some order in the output)
    */
    let chunk_size: u64 = 100;

    let next_start = Arc::new(AtomicU64::new(1));
    let remaining = Arc::new(AtomicUsize::new(results_needed));
    let stop_flag = Arc::new(AtomicBool::new(false));

    let mut handles = Vec::new();

    for _ in 0..num_threads {
        let next_start = Arc::clone(&next_start);
        let remaining = Arc::clone(&remaining);
        let stop_flag = Arc::clone(&stop_flag);

        let handle = thread::spawn(move || {
            while !stop_flag.load(Ordering::Relaxed) {
                let start = next_start.fetch_add(chunk_size, Ordering::Relaxed);
                let end = start + chunk_size;

                for num in start..end {
                    if stop_flag.load(Ordering::Relaxed) {
                        break;
                    }

                    let mut hasher = Sha256::new();
                    hasher.update(num.to_string().as_bytes());
                    let hash_result = hasher.finalize();
                    let hash_hex = format!("{:x}", hash_result);

                    if hash_hex.ends_with(&"0".repeat(num_zeros)) {
                        let prev = remaining.fetch_sub(1, Ordering::SeqCst);
                        
                        if prev > 0 {
                            println!("{}, \"{}\"", num, hash_hex);
                            
                            if prev == 1 {
                                stop_flag.store(true, Ordering::Relaxed);
                                break;
                            }
                        }
                    }
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}