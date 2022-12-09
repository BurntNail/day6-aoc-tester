#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use std::collections::HashMap;

pub fn old<const MSG_LEN: usize> (input: &str) -> usize { //4 for p1, 14 for p2
    let input: Vec<_> = input.chars().enumerate().collect();

    let mut end_index = 0;

    for window in input.windows(MSG_LEN) {
        let mut containers: HashMap<char, u32> = HashMap::new();
        for (_, c) in window {
            *containers.entry(*c).or_default() += 1;
        }

        if containers.iter().all(|(_, no)| no == &1) {
            end_index = window[MSG_LEN - 1].0 + 1; 
            break;
        }
    }


    end_index
}

pub fn new<const MSG_LEN: usize> (input: &str) -> usize { //4 for p1, 14 for p2
    let input: Vec<_> = input.chars().enumerate().collect();
    let mut end_index = 0;

    'outer: for window in input.windows(MSG_LEN) {

        let mut chars = 0_u32;
        for (wi, (i, c)) in window.iter().enumerate() {
            let new = chars | (1 << (*c as u8 - b'a'));
            if new == chars {
                break; 
            }

            if wi == MSG_LEN - 1 {
                end_index = i + 1; 
                break 'outer;
            }
            chars = new;
        }
    }


    end_index
}

pub fn newer <const MSG_LEN: usize> (input: &str) -> usize { //4 for p1, 14 for p2
    let input: Vec<_> = input.chars().enumerate().collect();
    let mut end_index = 0;

    for window in input.windows(MSG_LEN) {

        let mut chars = 0_u32;
        for (_, c) in window.iter() {
            chars |= 1 << (*c as u8 - b'a');
        }
        if chars.count_ones() == MSG_LEN as u32 {
            end_index = window[MSG_LEN - 1].0 + 1;
            break;
        }
    }


    end_index
}

pub mod amos;