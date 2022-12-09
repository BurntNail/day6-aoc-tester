pub fn marker_pos<const SEQUENCE_SIZE: usize>(input: &str) -> Option<usize> {
    assert!(input.len() > SEQUENCE_SIZE);

    let mut state = [0u8; 256];
    for b in &input.as_bytes()[..SEQUENCE_SIZE] {
        state[*b as usize] += 1;
    }
    if state.iter().all(|&x| x <= 1) {
        return Some(0);
    }

    for (index, window) in input.as_bytes().windows(SEQUENCE_SIZE + 1).enumerate() {
        let removed = window[0];
        let added = window[SEQUENCE_SIZE];
        state[removed as usize] -= 1;
        state[added as usize] += 1;

        if state.iter().all(|&x| x <= 1) {
            return Some(index + 1);
        }
    }

    None
}