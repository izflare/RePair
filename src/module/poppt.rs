extern crate bit_vec;

use bit_vec::BitVec;
use super::{cfg::*};

#[derive(Debug)]
pub struct POPPT {pub bit: BitVec, pub label: Vec<u32>, pub terminal: Vec<u8>}

impl POPPT {
    pub fn new() -> Self {
        Self {bit: BitVec::new(), label: Vec::new(), terminal: Vec::new(),}
    }

    pub fn to_grammar(&self, g: &mut Grammar) -> () {
        let mut stack: Vec<u32> = Vec::new();
        let mut var: u32 = self.terminal.len() as u32 + 1;
        g.terminal = self.terminal.clone();
        let mut i = 0;
        // for b in &self.bit {
        //     if b {
        //         if right_side.len() > 0 {
        //             g.rule.push(right_side.iter().rev().map(|x| *x).collect::<Vec<u32>>());
        //             stack.push(var);
        //             var += 1;
        //             right_side = Vec::new();
        //         }
        //         else {
        //             stack.push(self.label[i]);
        //             i += 1;
        //         }
        //     }
        //     else {right_side.push(stack.pop().unwrap());}
        // }
        for b in &self.bit {
            if b {
                // let mut right_side: Vec<u32> = Vec::new();
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                g.rule.push(vec![b, a]);
                // right_side.push(stack.pop().unwrap());
                // right_side.push(stack.pop().unwrap());
                // g.rule.push(right_side.iter().rev().map(|x| *x).collect::<Vec<u32>>());
                stack.push(var);
                var += 1;
            }
            else {
                stack.push(self.label[i]);
                i += 1;
            }
        }
        g.sequence = vec![var - 1];
        // println!("g: {:?}", g);
    }
}


