#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn get_seed(dec1: u32, dec2: u32, end1: u32) -> u32 {
    let init_seed_hash: u64 = (dec2 as u64) << 32 | dec1 as u64;
    let end_seed_hash: u64 = end1 as u64;

    let mut actual_seed = 0;
    // let mut game_seed_xor: u64 = 0;
    let magic: u64 = 0x6AC690C5;
    let offset: u64 = 666;

    let divisor: u64 = 2 << 16 - 1;
    let mut modifier: u64 = 0;

    for try_seed in 0..divisor {
        let seed_result: u64 = (try_seed as u64 * magic + offset) & 0xFFFFFFFF;
        if seed_result % divisor == end_seed_hash % divisor {
            modifier = try_seed;
        }
    }

    for i in 0..(u32::MAX / divisor as u32) {
        let try_seed: u64 = modifier + i as u64 * divisor;
        let seed_result: u64 = ((try_seed * magic + offset)  & 0xFFFFFFFF).into();

        if seed_result == end_seed_hash {
            // game_seed_xor = init_seed_hash ^ try_seed;
            actual_seed = try_seed;
        }
    }
    // println!("XOR: {} GameSeed: {}", game_seed_xor, actual_seed);
    return actual_seed.try_into().unwrap()
    
}


#[cfg(test)]
mod tests {
    use crate::get_seed;

    #[test]
    fn test_seed_decrypt() {
        let seed = get_seed(2817861018,2714388087,3262333376);
        assert_eq!(seed, 741378286);
    }
}
