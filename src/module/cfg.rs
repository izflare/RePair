use super::{poppt::*};

#[derive(Debug)]
pub struct Grammar {pub terminal: Vec<u8>, pub rule: Vec<Vec<u32>>, pub sequence: Vec<u32>,}

impl Grammar {
    pub fn new() -> Self {
        Self {terminal: Vec::new(), rule: Vec::new(), sequence: Vec::new(),}
    }

    pub fn derive(&self, w: &mut Vec<u8>) -> () {
        // zero is a special symbol
        fn dfs(i: usize, z: &Vec<u8>, g: &Vec<Vec<u32>>, w: &mut Vec<u8>) -> () {
            if i < z.len() + 1 {w.push(z[i - 1]);}
            else {
                let rs = &g[i - z.len() - 1];
                for e in rs {
                    dfs(*e as usize, z, g, w);
                }
            }
        }

        for c in &self.sequence {dfs(*c as usize, &self.terminal, &self.rule, w);}
    }


    pub fn to_poppt(&self, p: &mut POPPT) -> () {

        let mut m: Vec<Option<u32>> = vec![None; self.terminal.len() + self.rule.len()];
        let mut var: u32 = self.terminal.len() as u32 + 1;
        fn dfs(i: usize, z: &Vec<u8>, g: &Vec<Vec<u32>>, p: &mut POPPT, m: &mut Vec<Option<u32>>, var: &mut u32) -> () {
            if let Some(x) = m[i - 1] {
                p.label.push(x);
                p.bit.push(false);
            }
            else {
                if i < z.len() + 1 {
                    m[i - 1] = Some(p.terminal.len() as u32 + 1);
                    p.label.push(p.terminal.len() as u32 + 1);
                    p.bit.push(false);
                    p.terminal.push(z[i - 1]);
                }
                else {
                    let rs = &g[i - z.len() - 1];
                    for e in rs {
                        dfs(*e as usize, z, g, p, m, var);
                    }
                    p.bit.push(true);
                    m[i - 1] = Some(*var);
                    *var += 1;
                }
            }
        }

        dfs(self.sequence[0] as usize, &self.terminal, &self.rule, p, &mut m, &mut var);
        for c in 1..self.sequence.len() {
            let i = self.sequence[c] as usize;
            dfs(i, &self.terminal, &self.rule, p, &mut m, &mut var);
            p.bit.push(true);
            var += 1;
        }
    }
}


