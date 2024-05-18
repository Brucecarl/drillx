extern "C" {
    pub static BATCH_SIZE: u32;
    pub fn hash(challenge: *const u8, nonce: *const u8, out: *mut u64);
    pub fn solve_all_stages(hashes: *const u64, out: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INDEX_SPACE: usize = 65536;

    fn hashspace_size() -> usize {
        unsafe { BATCH_SIZE as usize * INDEX_SPACE }
    }

    #[test]
    fn test_gpu() {
        let challenge = [255; 32];
        let nonce = [2; 8];
        let mut hashes = vec![0u64; hashspace_size()];
        unsafe {
            hash(
                challenge.as_ptr(),
                nonce.as_ptr(),
                hashes.as_mut_ptr() as *mut u64,
            );
            // for i in 0..BATCH_SIZE as usize {
            let i = 32;
            let mut digest = [0u8; 16];
            let batch_start = hashes.as_ptr().add(i * INDEX_SPACE);
            println!("{} hash: {}", i, *batch_start);
            solve_all_stages(batch_start, digest.as_mut_ptr());
            let solution = crate::Solution::new(digest, nonce);
            // println!("Digest: {:?}", digest);
            println!("{} is valid: {}", i, solution.is_valid(&challenge));
            // }
            // assert!(false);
        }
    }
}
