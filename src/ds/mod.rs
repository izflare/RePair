use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
pub struct Bigram {pub left: u32, pub right: u32,}

#[derive(Clone)]
pub struct Bucket {pub val: Option<u32>, pub prev: Option<usize>, pub next: Option<usize>,}

pub trait PairArray : std::ops::Index<usize> {
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
        // // q の関数
        // // Record をリストから切り離す
        // fn out_rec(q: &mut Vec<(Option<*mut Rec>, Option<*mut Rec>)>, r: &mut Rec) {
        //     //{{{
        //     if r.prev == None {
        //         q[r.freq].0 = r.next;
        //     }
        //     else {
        //         unsafe {
        //             let pr: &mut Rec = &mut *r.prev.unwrap();
        //             pr.next = r.next;
        //         }
        //     }
        //
        //     if r.next == None {
        //         q[r.freq].1 = r.prev;
        //     }
        //     else {
        //         unsafe {
        //             let nx: &mut Rec = &mut *r.next.unwrap();
        //             nx.prev = r.prev;
        //         }
        //     }
        //     r.prev = None;
        //     r.next = None;
        //     //}}}
        // }
        //
        // // Record をリストの末尾に追加
        // fn in_rec(q: &mut Vec<(Option<*mut Rec>, Option<*mut Rec>)>, r: &mut Rec) {
        //     //{{{
        //     let ptr: *mut Rec = &mut *r;
        //     if q[r.freq].1 != None {
        //         unsafe {
        //             let tail: &mut Rec = &mut *q[r.freq].1.unwrap();
        //             tail.next = Some(ptr);
        //             r.prev = Some(tail);
        //         }
        //     }
        //     else {
        //         q[r.freq].0 = Some(ptr);
        //         r.prev = None;
        //     }
        //     r.next = None;
        //     q[r.freq].1 = Some(ptr);
        //     //}}}
        // }
        //
        // // unsafe fn pairsort(q: &mut Vec<(Option<*mut Rec>, Option<*mut Rec>)>, f: usize,
        // //              a: &Vec<(Option<u32>, Option<usize>, Option<usize>)>) -> () {
        // //     let mut lv: Vec<&mut Rec> = Vec::new();
        // //     // fn lv_push(r: &mut Rec,
        // //     //     q: &mut Vec<(Option<*mut Rec>, Option<*mut Rec>)>, f: usize, lv: &mut Vec<&mut Rec>) -> () {
        // //     //     let nwrap = r.next;
        // //     //     out_rec(q, r);
        // //     //     lv.push(r);
        // //     //     if let Some(n) = nwrap {lv_push(&mut * n, q, f, lv);}
        // //     // }
        // //     // lv_push(&mut *q[f].0.unwrap(), q, f, &mut lv);
        // //     let mut r = *q[f].0.unwrap();
        // //     loop {
        // //         let nwrap = r.next;
        // //         out_rec(q, &mut r);
        // //         lv.push(&mut r);
        // //         if let Some(n) = nwrap {r = *n;}
        // //         else {break;}
        // //     }
        // //     for r in &lv {
        // //         in_rec(q, *r);
        // //     }
        // //     // println!("{:?}", lv);
        // // }
        // //
        // // unsafe fn printlist(q: &mut Vec<(Option<*mut Rec>, Option<*mut Rec>)>, f: usize, 
        // //              a: &Vec<(Option<u32>, Option<usize>, Option<usize>)>) -> () {
        // //     unsafe fn print_pair(r: &mut Rec,
        // //              a: &Vec<(Option<u32>, Option<usize>, Option<usize>)>) -> () {
        // //         println!("{:?}, ", get_bg(&a, r.loc));
        // //         if let Some(n) = r.next {print_pair(&mut *n, a);}
        // //     }
        // //     print_pair(&mut *q[f].0.unwrap(), a);
        // // }
        // //}}}
        //
