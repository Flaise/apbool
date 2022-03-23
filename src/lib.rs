use std::ops::{Deref, BitOrAssign};
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
}
