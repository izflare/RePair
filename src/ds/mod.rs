use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
pub struct Bigram {pub left: u32, pub right: u32,}

#[derive(Clone)]
pub struct Bucket {pub val: Option<u32>, pub prev: Option<usize>, pub next: Option<usize>,}

pub trait PairArray: std::ops::Index<usize> {
    fn rgh_pos(&self, i: usize) -> Option<usize>;
    fn lft_pos(&self, i: usize) -> Option<usize>;
    fn rgh_bg(&self, i: usize) -> Option<Bigram>;
    fn lft_bg(&self, i: usize) -> Option<Bigram>;
}

impl PairArray for Vec<Bucket> {
    //{{{
    fn rgh_pos(&self, i: usize) -> Option<usize> {
        if self[i].val == None {self[i].next}
        else if i + 1 < self.len() {Some(i + 1)}
        else {None}
    }

    fn lft_pos(&self, i: usize) -> Option<usize> {
        if self[i].val == None {self[i].prev}
        else if i > 0 {Some(i - 1)}
        else {None}
    }

    fn rgh_bg(&self, i: usize) -> Option<Bigram> {
        if let Some(rgh) = self.rgh_pos(i) {
            match (self[i].val, self[rgh].val) {
                (Some(x), Some(y)) => Some(Bigram {left: x, right: y,}),
                _ => None
            }
        }
        else {None}
    }

    fn lft_bg(&self, i: usize) -> Option<Bigram> {
        if let Some(lft) = self.lft_pos(i) {
            match (self[lft].val, self[i].val) {
                (Some(x), Some(y)) => Some(Bigram {left: x, right: y,}),
                _ => None
            }
        }
        else {None}
    }
    //}}}
}

pub struct Record {pub loc: usize, pub freq: usize, pub prev: Option<*mut Record>, pub next: Option<*mut Record>}

pub fn create_ds(a: &mut Vec<Bucket>, h: &mut HashMap<Bigram, *mut Record>, z: &mut Vec<u8>, s: &Vec<u8>) -> () {
    //{{{
    let mut d: HashMap<u8, u32> = HashMap::new();
    let mut var: u32 = 1;
    let mut f: usize = 1;
    for i in 0..s.len() {
        if d.contains_key(&s[i]) {
            a[i].val = Some(*d.entry(s[i]).or_insert(var));
        }
        else {
            d.insert(s[i], var);
            z.push(s[i]);
            var += 1;
        }
    }
    for i in (0..s.len()-1).rev() {
        if let Some(bg) = a.rgh_bg(i) {
            if h.contains_key(&bg) {
                if let Some(ref_ptr) = h.get(&bg) { 
                    unsafe {
                        a[i].next = Some((**ref_ptr).loc);
                        a[(**ref_ptr).loc].prev = Some(i);
                        (**ref_ptr).loc = i;
                        (**ref_ptr).freq += 1;
                        if f < (**ref_ptr).freq {f = (**ref_ptr).freq;}
                    }
                }
            }
            else {h.insert(bg, Box::into_raw(Box::new(Record {loc: i, freq: 1, prev: None, next: None})));}
        }
    }
    //}}}
}

pub struct List {pub head: Option<*mut Record>, pub tail: Option<*mut Record>}

pub trait FreqTable: std::ops::Index<usize> {
    fn top(&self, f: usize) -> Option<*mut Record>;
    unsafe fn push(&mut self, r: *mut Record) -> ();
    unsafe fn pop(&mut self, r: *mut Record) -> ();
}

impl FreqTable for Vec<List> {
    //{{{
    fn top(&self, f: usize) -> Option<*mut Record> {
        return self[f].head
    }

    unsafe fn push(&mut self, r: *mut Record) -> () {
        let f = (*r).freq;
        if let Some(tail) = self[f].tail {
            (*tail).next = Some(r);
            (*r).prev = Some(tail);
            self[f].tail = Some(r);
        }
        else {
            self[f].head = Some(r);
            self[f].tail = Some(r);
        }
    }

    unsafe fn pop(&mut self, r: *mut Record) -> () {
        let f = (*r).freq;
        match ((*r).prev, (*r).next) {
            (Some(x), Some(y)) => {(*x).next = Some(y); (*y).prev = Some(x);},
            (Some(x), None) => {(*x).next = None; self[f].tail = Some(x);},
            (None, Some(y)) => {(*y).prev = None; self[f].head = Some(y);},
            (None, None) => {self[f].head = None; self[f].tail = None;},
        }
        (*r).prev = None;
        (*r).next = None;
    }
    //}}}
}

        // // 頻度表を作成
        // // (head, tail)
        // let mut q: Vec<(Option<*mut Rec>, Option<*mut Rec>)> = vec![(None, None); f+1];
        // //{{{
        // for e in &k {
        //     let v = h.get(e).unwrap();
        //     unsafe {
        //         let r: &mut Rec = &mut **v;
        //         in_rec(&mut q, r);
        //     }
        // }
        //
