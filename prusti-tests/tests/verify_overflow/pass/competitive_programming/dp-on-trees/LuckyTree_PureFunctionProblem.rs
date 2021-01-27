// compile-flags: -Passert_timeout=120000
// https://codeforces.com/blog/entry/20935

#![feature(box_patterns)]
#![feature(box_syntax)]

use prusti_contracts::*;
use std::ptr;

pub struct Tree {
    n: isize,
    idx: isize,
    isLucky: bool,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl Tree {
    #[trusted]
    #[pure]
    #[ensures (result == self.isLucky)]
    pub fn isLucky(&self) -> bool {
        self.isLucky
    }

    #[trusted]
    #[pure]
    #[ensures (result == self.n)]
    pub fn n(&self) -> isize {
        self.n
    }

    #[trusted]
    #[pure]
    #[ensures (result == self.idx)]
    #[ensures(result >= 0 && result < self.n)]
    pub fn idx(&self) -> isize {
        self.idx
    }

    #[trusted]
    #[requires(i  >=  0  && i < nn)]
    #[ensures (result.isLucky() == c)]
    #[ensures (result.idx() == i)]
    #[ensures(same_n(&result))]
    pub fn new(nn: isize, i: isize, c: bool, l: Option<Box<Tree>>, r: Option<Box<Tree>>) -> Self {
        Tree {
            n: nn,
            idx: i,
            isLucky: c,
            left: l,
            right: r,
        }
    }
}

#[pure]
pub fn same_n(node: &Tree) -> bool {
    let mut result = true;
    match &(*node).left {
        None => {}
        Some(box l) => {
            result &= same_n(&l);
            result &= (node.n == l.n);
        }
    }

    match &(*node).right {
        None => {}
        Some(box r) => {
            result &= same_n(&r);
            result &= (node.n == r.n);
        }
    }
    result
}

pub struct VecWrapperI32 {
    _ghost_size: usize,
    v: Vec<isize>,
}

impl VecWrapperI32 {
    #[trusted]
    #[pure]
    #[ensures (0 <= result)]
    pub fn len(&self) -> isize {
        self._ghost_size as isize
    }

    #[trusted]
    #[requires(size > 0)]
    #[ensures (result.len() == size)]
    #[ensures (forall(|i: isize| (0 <= i && i < result.len()) ==> result.lookup(i) == 0))]
    pub fn new(size: isize) -> Self {
        Self {
            _ghost_size: size as usize,
            v: vec![0; size as usize],
        }
    }

    #[trusted]
    #[pure]
    #[requires (0 <= index && index < self.len())]
    pub fn lookup(&self, index: isize) -> isize {
        self.v[index as usize]
    }

    #[trusted]
    #[requires(0 <= idx && idx < self.len())]
    #[ensures(self.len() == old(self.len()))]
    #[ensures(self.lookup(idx) == value)]
    #[ensures(forall(|i: isize|
        (0 <= i && i < self.len() && i != idx) ==>
        self.lookup(i) == old(self.lookup(i))))]
    pub fn set(&mut self, idx: isize, value: isize) -> () {
        self.v[idx as usize] = value
    }
}

#[pure]
#[ensures (result >= a && result >= b)]
#[ensures (result == a || result == b)]
fn max(a: isize, b: isize) -> isize {
    if a > b {
        a
    } else {
        b
    }
}

#[trusted]
#[pure]
#[ensures(result == a + b)]
fn add(a: isize, b: isize) -> isize {
    a + b
}

// Naive Solution

#[pure]
#[requires(same_n(node))]
fn sub_size(node: &Tree) -> isize {
    let mut sz = 1isize;
    match &(*node).left {
        None => {}
        Some(box l) => {
            sz += sub_size(l);
        }
    }

    match &(*node).right {
        None => {}
        Some(box r) => {
            sz += sub_size(r);
        }
    }
    sz
}

#[pure]
#[requires(same_n(node))]
#[ensures(node.isLucky() ==> result == sub_size(node))]
#[ensures(!node.isLucky() ==> result == down_lucky(node))]
fn calc_down_lucky(node: &Tree) -> isize {
    if node.isLucky() {
        sub_size(node)
    } else {
        down_lucky(node)
    }
}

#[pure]
#[requires(same_n(node))]
fn down_lucky(node: &Tree) -> isize {
    let mut d = 0isize;
    match &(*node).left {
        None => {}
        Some(box l) => {
            d += calc_down_lucky(l);
        }
    }

    match &(*node).right {
        None => {}
        Some(box r) => {
            d += calc_down_lucky(r);
        }
    }
    d
}

#[trusted]
#[ensures(false)]
fn assume_false() {}

// DP Solution

#[pure]
#[requires(same_n(node))]
#[requires(same_n(child))]
fn compute_child_answer(node: &Tree, child: &Tree, upLucky: isize) -> isize {
    if child.isLucky() {
        dfs2_compute(child, node.n() - sub_size(node))
    } else {
        dfs2_compute(child, upLucky + down_lucky(node) - down_lucky(child))
    }
}

#[pure]
#[requires(same_n(node))]
fn dfs2_compute(node: &Tree, upLucky: isize) -> isize {
    let d1 = down_lucky(node);
    let tot = upLucky + d1;
    let mut result = tot * (tot - 1);
    match &(*node).left {
        None => {}
        Some(box l) => {
            result += compute_child_answer(node, l, upLucky);
        }
    }
    match &(*node).right {
        None => {}
        Some(box r) => {
            result += compute_child_answer(node, r, upLucky);
        }
    }
    result
}

fn main() {}
