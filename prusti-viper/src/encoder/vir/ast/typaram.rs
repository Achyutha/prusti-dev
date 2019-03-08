// © 2019, ETH Zurich
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use regex::Regex;
use std::collections::HashMap;

pub struct Substs {
    regex: Regex,
    repls: HashMap<String, String>,
}

fn escape_dollars(s: &str) -> String {
    s.replace('$', "\\$")
}

impl Substs {
    pub fn learn(from: &str, to: &str) -> Self {
        // construct repls_regex
        lazy_static! {
            static ref re: Regex = Regex::new("(__TYPARAM__\\$(.*?)\\$__)").unwrap();
        }
        let mut repls_regex_str = String::new();
        repls_regex_str.push('^');
        let mut typarams = Vec::new();
        let mut last = 0;
        for matsh in re.find_iter(from) {
            repls_regex_str.push_str(&escape_dollars(&from[last..matsh.start()]));
            repls_regex_str.push_str("(.*?)");
            typarams.push(matsh.as_str().to_string());
            last = matsh.end();
        }
        repls_regex_str.push_str(&escape_dollars(&from[last..]));
        repls_regex_str.push('$');
        // use repls_regex to find typaram replacements
        let mut repls = HashMap::new();
        let repls_regex = Regex::new(&repls_regex_str).unwrap();
        let captures = repls_regex.captures(to).unwrap();
        for i in 1..captures.len() {
            let from = typarams[i-1].to_string();
            let to = captures.get(i).unwrap().as_str();
            let old = repls.insert(from, to.to_string());
            if let Some(x) = old {
                assert!(to == x);
            }
        }
        Substs {
            regex: re.clone(),
            repls,
        }
    }

    pub fn apply(&self, inner1: &str) -> String {
        let mut newstr = String::new();
        let mut last = 0;
        for matsh in self.regex.find_iter(inner1) {
            newstr.push_str(&inner1[last..matsh.start()]);
            newstr.push_str(&self.repls[matsh.as_str()]);
            last = matsh.end();
        }
        newstr.push_str(&inner1[last..]);
        newstr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(outer1: &str, outer2: &str, inner1: &str, inner2: &str) {
        let substs = Substs::learn(outer1, outer2);
        let inner2_gen = substs.apply(inner1);
        assert_eq!(inner2_gen, inner2);
    }

    #[test]
    pub fn test1() {
        let outer1 = "ref$m_generics_basic_3$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$Y$__$_end_";
        let outer2 = "ref$m_generics_basic_3$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$Z$__$_end_";
        let inner1 = "m_generics_basic_3$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$Y$__$_end_";
        let inner2 = "m_generics_basic_3$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$Z$__$_end_";
        test(outer1, outer2, inner1, inner2);
    }

    #[test]
    fn test2() {
        let outer1 = "ref$m_generics_basic_7$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$A$__$_sep_$__TYPARAM__$B$__$_end_";
        let outer2 = "ref$m_generics_basic_7$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$B$__$_sep_$__TYPARAM__$A$__$_end_";
        let inner1 = "m_generics_basic_7$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$A$__$_sep_$__TYPARAM__$B$__$_end_";
        let inner2 = "m_generics_basic_7$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$B$__$_sep_$__TYPARAM__$A$__$_end_";
        test(outer1, outer2, inner1, inner2);
    }

    #[test]
    fn test3() {
        let outer1 = "m_generics_basic_6$$Foo$opensqu$0$closesqu$$_beg_$__TYPARAM__$C$__$_end_";
        let outer2 = "m_generics_basic_6$$Foo$opensqu$0$closesqu$$_beg_$u128$_end_";
        let inner1 = "m_generics_basic_6$$BarBaz$opensqu$0$closesqu$$_beg_$__TYPARAM__$C$__$_end_";
        let inner2 = "m_generics_basic_6$$BarBaz$opensqu$0$closesqu$$_beg_$u128$_end_";
        test(outer1, outer2, inner1, inner2);
    }

    #[test]
    fn test4() {
        let outer1 = "ref$m_generics_basic_4$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$A$__$_sep_$__TYPARAM__$B$__$_end_";
        let outer2 = "ref$m_generics_basic_4$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$C$__$_sep_$i16$_end_";
        let inner1 = "m_generics_basic_4$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$A$__$_sep_$__TYPARAM__$B$__$_end_";
        let inner2 = "m_generics_basic_4$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$C$__$_sep_$i16$_end_";
        test(outer1, outer2, inner1, inner2);
    }

    #[test]
    fn test5() {
        let outer1 = "ref$m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$A$__$_sep_$m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$B$__$_sep_$i32$_sep_$__TYPARAM__$C$__$_end_$_sep_$__TYPARAM__$D$__$_end_";
        let outer2 = "ref$m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$i8$_sep_$i32$_sep_$u8$_end_$_sep_$m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$i16$_sep_$i32$_sep_$i64$_end_$_sep_$m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$isize$_sep_$i32$_sep_$usize$_end_$_end_";
        let inner1 = "m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$B$__$_sep_$i32$_sep_$__TYPARAM__$C$__$_end_";
        let inner2 = "m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$i16$_sep_$i32$_sep_$i64$_end_";
        test(outer1, outer2, inner1, inner2);
    }

    #[test]
    fn test6() {
        let outer1 = "ref$m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$A$__$_sep_$m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$B$__$_sep_$i32$_sep_$__TYPARAM__$C$__$_end_$_sep_$__TYPARAM__$D$__$_end_";
        let outer2 = "ref$m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$i8$_sep_$i32$_sep_$u8$_end_$_sep_$m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$i16$_sep_$i32$_sep_$i64$_end_$_sep_$m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$isize$_sep_$i32$_sep_$usize$_end_$_end_";
        let inner1 = "m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$A$__$_sep_$m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$__TYPARAM__$B$__$_sep_$i32$_sep_$__TYPARAM__$C$__$_end_$_sep_$__TYPARAM__$D$__$_end_";
        let inner2 = "m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$i8$_sep_$i32$_sep_$u8$_end_$_sep_$m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$i16$_sep_$i32$_sep_$i64$_end_$_sep_$m_generics_basic_5$$Number$opensqu$0$closesqu$$_beg_$isize$_sep_$i32$_sep_$usize$_end_$_end_";
        test(outer1, outer2, inner1, inner2);
    }
}
