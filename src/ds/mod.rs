// use std::collections::HashMap;

struct Bigram {left: u32, right: u32,}

struct Bucket {val: Option<u32>, prev: Option<usize>, next: Option<usize>,}

trait PairArray : std::ops::Index<usize> {
    fn rgh_pos(&self, i: usize) -> Option<usize>;
    fn lft_pos(&self, i: usize) -> Option<usize>;
    fn rgh_bg(&self, i: usize) -> Option<Bigram>;
    fn lft_bg(&self, i: usize) -> Option<Bigram>;
}

impl PairArray for Vec<Bucket> {
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
}



        // // preprocessing
        // // 終端記号を変数に置換して，文字列を配列に格納
        // // each tuple is (0: char, 1: prev, 2: next)
        // let mut a: Vec<(Option<u32>, Option<usize>, Option<usize>)> = vec![(None, None, None); s.len()]; 
        // let mut z: Vec<u8> = Vec::new();
        // //{{{
        // {
        //     // let mut d: HashMap<char, usize> = HashMap::new();
        //     let mut d: HashMap<u8, u32> = HashMap::new();
        //     let mut x = 1;
        //     for i in 0..s.len() {
        //         if d.contains_key(&s[i]) {
        //             let e = d.get(&s[i]);
        //             a[i] = (Some(*e.unwrap()), None, None);
        //         }
        //         else {
        //             d.insert(s[i], x);
        //             a[i] = (Some(x), None, None);
        //             x += 1;
        //             z.push(s[i]);
        //         }
        //     }
        // }
        //
        //
        // //}}}
        //
        // // bigramの位置をつなぎながらハッシュ表を作成
        // struct Rec {loc: usize, freq: usize, prev: Option<*mut Rec>, next: Option<*mut Rec>};
        // let mut h: HashMap<(u32, u32), *mut Rec> = HashMap::new();
        // let mut f: usize = 1;
        // let mut k: Vec<(u32, u32)> = Vec::new();
        // //{{{
        // for i in (0..s.len()-1).rev() {
        //     let b = (a[i].0.unwrap(), a[i+1].0.unwrap());
        //     if h.contains_key(&b) {
        //         unsafe {
        //             let mut r: &mut Rec = &mut **(h.get(&b).unwrap());
        //             a[i].2 = Some(r.loc);
        //             a[r.loc].1 = Some(i);
        //             r.loc = i;
        //             r.freq += 1;
        //             if f < r.freq {f = r.freq;}
        //         }
        //     }
        //     else {
        //         let r = Box::new(Rec {loc: i, freq: 1, prev: None, next: None});
        //         let x: *mut Rec = Box::into_raw(r);
        //         h.insert(b, x);
        //         k.push(b);
        //     }
        // }
        // //}}}
        //
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
