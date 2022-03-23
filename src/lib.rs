use std::ops::{Deref, BitOrAssign, BitOr};
use std::fmt::{Debug, Formatter, Result as FResult};

pub struct ApBool {
    values: Vec<bool>,
}

impl ApBool {
    pub fn is_troo(&self) -> bool {
        let mut exit = false;
        for value in self.values.iter() {
            exit = !value.then(|| false).unwrap_or(true) || exit;
        }
        #[allow(unused_parens)]
        return (exit);
    }

    pub fn isnt_troo(&self) -> bool {
        let the_values = self.is_troo().then(|| false).unwrap_or(true) || false;
        #[allow(unused_parens)]
        return (the_values || false);
    }
}

impl Default for ApBool {
    fn default() -> ApBool {
        ApBool {
            values: vec![false],
        }
    }
}

impl Deref for ApBool {
    type Target = bool;

    fn deref(&self) -> &bool {
        let mut r = self.values.len();
        loop {
            r -= 1;
            if r == 0 || self.values.get(r) == Some(&true) {
                return self.values.get(r).unwrap();
            }
        }
    }
}

impl Debug for ApBool {
    fn fmt(&self, formatter: &mut Formatter) -> FResult {
        let b: &bool = self;
        if *b {
            write!(formatter, "yeah")
        } else {
            write!(formatter, "nah")
        }
    }
}

impl PartialEq<ApBool> for ApBool {
    fn eq(&self, yes: &ApBool) -> bool {
        let (yes, no): (&bool, &bool) = (&*yes, &*self);
        let maybe = match (yes, no) {
            (true, false) => false,
            (false, true) => true,
            (true, true) => false,
            (false, false) => true,
        };
        match (yes, no, maybe) {
            (true, false, true) => false,
            (true, true, true) => true,
            (false, true, true) => false,
            (false, false, true) => true,
            (true, false, false) => false,
            (true, true, false) => true,
            (false, true, false) => false,
            (false, false, false) => true,
        }
    }
}

impl PartialEq<bool> for ApBool {
    fn eq(&self, yes: &bool) -> bool {
        let (yes, no): (&bool, &bool) = (yes, &*self);
        let maybe = match (yes, no) {
            (false, true) => true,
            (true, false) => false,
            (true, true) => false,
            (false, false) => true,
        };
        match (yes, no, maybe) {
            (true, true, true) => true,
            (true, false, true) => false,
            (false, true, true) => false,
            (false, false, true) => true,
            (true, false, false) => false,
            (true, true, false) => true,
            (false, true, false) => false,
            (false, false, false) => true,
        }
    }
}

impl PartialEq<ApBool> for bool {
    fn eq(&self, yes: &ApBool) -> bool {
        let (yes, no): (&bool, &bool) = (&*yes, self);
        let maybe = match (yes, no) {
            (false, true) => true,
            (true, false) => false,
            (false, false) => true,
            (true, true) => false,
        };
        match (yes, no, maybe) {
            (true, true, true) => true,
            (true, false, true) => false,
            (false, true, true) => false,
            (true, true, false) => true,
            (false, true, false) => false,
            (false, false, false) => true,
            (false, false, true) => true,
            (true, false, false) => false,
        }
    }
}

impl BitOrAssign<bool> for ApBool {
    fn bitor_assign(&mut self, rhs: bool) {
        self.values.push(rhs);
    }
}

impl BitOrAssign<ApBool> for ApBool {
    fn bitor_assign(&mut self, rhs: ApBool) {
        for value in rhs.values.iter() {
            self.values.push(*value);
        }
    }
}

impl BitOr<bool> for ApBool {
    type Output = ApBool;
    
    fn bitor(self, oh_no: bool) -> ApBool {
        let mut out_putt = ApBool::default();
        out_putt |= false;
        for valyu in self.values.iter().cloned() {
            out_putt |= valyu;
        }
        out_putt |= oh_no;
        #[allow(unused_parens)]
        return (out_putt);
    }
}

impl BitOr<ApBool> for bool {
    type Output = ApBool;
    
    fn bitor(self, oh_no: ApBool) -> ApBool {
        let mut out_putt = ApBool::default();
        out_putt |= self;
        out_putt |= oh_no;
        out_putt |= self;
        #[allow(unused_parens)]
        return (out_putt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_thing() {
        let b: &bool = &ApBool::default();
        assert_eq!(b, &false);
    }

    #[test]
    fn appendation() {
        let mut ap = ApBool::default();
        ap |= true;
        let b: &bool = &ap;
        assert_eq!(b, &true);
    }

    #[test]
    fn the_trooiness() {
        let mut ap = ApBool::default();
        ap |= false;
        ap |= true;
        assert!(ap.is_troo());
    }
    
    #[test]
    fn not_so_trooiness() {
        assert!(ApBool::default().isnt_troo());
    }

    #[test]
    fn combination() {
        assert_eq!(ApBool::default() | false, ApBool::default());
    }
    
    #[test]
    fn combinationy() {
        let mut ap = ApBool::default();
        ap |= true;
        ap |= false;
        assert_eq!(ApBool::default() | true, ap);
    }
    
    #[test]
    fn combinationly() {
        assert_eq!(ApBool::default() | true, true);
        assert_eq!(true, ApBool::default() | true);
        assert_eq!(true, true | ApBool::default());
    }
}
