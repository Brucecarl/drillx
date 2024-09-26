#![feature(duration_millis_float)]
use drillx::{
    cuda::{hash, solve_all_stages},
    Solution,
};
use num_cpus;
use std::{sync::Arc, thread, time::Instant};

const BATCH_SIZE: u32 = 512*4;
const INDEX_SPACE: usize = 65536;
fn hashspace_size() -> usize {
    BATCH_SIZE as usize * INDEX_SPACE
}

fn main() {

    let challenge = [255; 32];
    let nonce = 0u64;
    // let nonce=[2;8];
    println!("Start gpu perf for drillx:{} ...",nonce);
    let mut hashes = vec![0u64; hashspace_size()];
    unsafe {
        // Do compute heavy hashing on gpu
        let timer = Instant::now();
        hash(
            challenge.as_ptr(),
            nonce,
            hashes.as_mut_ptr() as *mut u64,
            BATCH_SIZE,
        );
        println!(
            "Gpu returned {} hashes in {} ms ,{}/s",
            BATCH_SIZE,
            timer.elapsed().as_millis(),
            (BATCH_SIZE as f32/timer.elapsed().as_millis_f32())*1000f32,
        );

        // Do memory heavy work on cpu
        // let num_threads = num_cpus::get();
        // let chunk_size = BATCH_SIZE as usize / num_threads;
        // let challenge = Arc::new(challenge);
        // let hashes = Arc::new(hashes);
        // let mut handles = vec![];
        // for t in 0..num_threads {
        //     let challenge = challenge.clone();
        //     let hashes = hashes.clone();
        //     // let nonce = u64::from_le_bytes(nonce);
        //     let handle = thread::spawn(move || {
        //         let start = t * chunk_size;
        //         let end = if t == num_threads - 1 {
        //             BATCH_SIZE as usize
        //         } else {
        //             start + chunk_size
        //         };
        //         let mut printed=false;
        //         for i in start..end {
        //             let mut digest = [0u8; 16];
        //             let mut sols = [0u8; 4];
        //             let batch_start = hashes.as_ptr().add(i * INDEX_SPACE);
        //             solve_all_stages(
        //                 batch_start,
        //                 digest.as_mut_ptr(),
        //                 sols.as_mut_ptr() as *mut u32,
        //             );
        //             if u32::from_le_bytes(sols).gt(&0) {
        //                 let solution = Solution::new(digest, (nonce + i as u64).to_le_bytes());
        //                 // assert!(solution.is_valid(&challenge));
        //                 if !solution.is_valid(&challenge)&&!printed{
        //                     printed=true;
        //                     println!("solution invalid:{},{}-{}",u32::from_le_bytes(sols),start,end);
        //                 }
        //             }
        //         }
        //     });
        //     handles.push(handle);
        // }

        // for handle in handles {
        //     handle.join().expect("Failed to join thread");
        // }
        println!(
            "Did {} hashes in {} ms",
            BATCH_SIZE,
            timer.elapsed().as_millis()
        );
    }
}
