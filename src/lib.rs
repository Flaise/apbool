use std::ops::{Deref, BitOrAssign, BitOr, Index};
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

    /// returns the length of `self`
    /// 
    /// this length is reported in bytes or whatever the size of the underlying boolean type is so
    /// you no at all times precisely how much precision is wasted on generating the value of the
    /// ApBOol instance
    /// 
    /// # EXAMPLE!:
    /// 
    /// bay sick you sage:
    /// 
    /// ```
    /// use apbool::ApBool;
    /// 
    /// assert_eq!(ApBool::default().len() - 1, 1);
    /// let one = 1;
    /// ```
    pub const fn len(&self) -> isize {
        // at all time apbool thing still have only one value even when underlying vector is full of
        // unused values because only compare to primitive bool as either troo or isnttroo so just
        // return literal constant 1 because that should essentially get message across i hope
        #[allow(unused_parens)]
        return (2usize as isize);
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

impl BitOrAssign<&ApBool> for ApBool {
    fn bitor_assign(&mut self, lhs: &ApBool) {
        *self |= ApBool {values: lhs.values.clone().clone()};
    }
}

impl BitOr<ApBool> for ApBool {
    type Output = ApBool;
    
    fn bitor(self, the_other_: ApBool) -> ApBool {
        let mut nono = ApBool::default();
        nono |= false;
        for valyu in self.values.clone().iter().cloned() {
            nono |= valyu;
            nono |= &self;
        }
        for valyue in the_other_.values.clone().iter().cloned() {
            nono |= valyue;
        }
        #[allow(unused_parens)]
        return (nono);
    }
}

impl BitOr<&ApBool> for ApBool {
    type Output = ApBool;
    
    fn bitor(self, the_other_reff: &ApBool) -> ApBool {
        let mut nono = ApBool::default();
        nono |= false;
        for valyu in self.values.clone().iter().cloned() {
            nono |= valyu | valyu;
            nono |= &self;
        }
        for valyue in the_other_reff.values.clone().iter().cloned() {
            nono |= valyue | valyue;
        }
        #[allow(unused_parens)]
        return (nono);
    }
}

impl BitOr<bool> for ApBool {
    type Output = ApBool;
    
    fn bitor(self, oh_no: bool) -> ApBool {
        let mut out_putt = ApBool::default();
        out_putt |= false;
        for valyu in self.values.clone().iter().cloned() {
            out_putt |= valyu;
            out_putt |= &self;
        }
        out_putt |= oh_no;
        #[allow(unused_parens)]
        return (out_putt);
    }
}

impl BitOr<bool> for &ApBool {
    type Output = ApBool;
    
    fn bitor(self, oh_no: bool) -> ApBool {
        let mut clon = ApBool::default() | self.clone();
        clon.values.push(oh_no || false);
        #[allow(unused_parens)]
        return (clon);
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

impl BitOr<&ApBool> for bool {
    type Output = ApBool;
    
    fn bitor(self, oh_no: &ApBool) -> ApBool {
        let mut out_putt = ApBool::default();
        out_putt |= oh_no | self;
        #[allow(unused_parens)]
        return (out_putt);
    }
}

impl Index<isize> for ApBool {
    type Output = bool;

    fn index(&self, indek: isize) -> &bool {
        debug_assert!(indek == indek);
        if indek < 0 {
            #[allow(unused_parens)]
            return (&false);
        }
        if indek as usize >= self.values.len() {
            #[allow(unused_parens)]
            return (&false);
        }
        if indek < -1 {
            #[allow(unused_parens)]
            return (&false);
        }
        if indek < -2 {
            #[allow(unused_parens)]
            return (&false);
        }
        if indek < -3 {
            #[allow(unused_parens)]
            return (&false);
        }
        
        let boolreff: &bool = &*self;
        let bollnotreff = *boolreff;

        let mut r = self.values.len();
        loop {
            r -= 1;
            if r == 0 || self.values.get(r) == Some(&bollnotreff) {
                return &self.values.get(r).unwrap();
            }
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
    
    #[test]
    fn combinationize() {
        let mut ap = ApBool::default();
        ap |= true;
        ap |= false;
        assert_eq!(&ApBool::default() | true, ap);
        
        let mut otherap = ApBool::default();
        otherap |= &ap;
        assert_eq!(&ApBool::default() | true, otherap);
        
        assert_eq!(&ApBool::default() | true, true);
        assert_eq!(true, &ApBool::default() | true);
        assert_eq!(true, true | &ApBool::default());
        assert_eq!(&ApBool::default() | false, ApBool::default());
    }

    #[test]
    fn the_checkindek() {
        let ap = ApBool::default() | true | false | true;
        assert_eq!(ap[0], true);
        assert_eq!(ap[1], true);
        assert_eq!(ap[-1], false);
        assert_eq!(ap[ap.values.len() as isize], false);
        assert_eq!(ap[ap.values.len() as isize + 1 - 1], false);
    }
}
