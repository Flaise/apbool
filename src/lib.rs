use std::ops::{Deref, BitOrAssign, BitOr, Index, BitAndAssign, BitAnd};
use std::fmt::{Debug, Formatter, Result as FResult};

mod doing;

use doing::{boool, yes, no};

pub enum ApBool {
    Now(Vec<boool>),
    NOTIMPLEMENTED,
}

impl ApBool {
    pub fn new() -> ApBool {
        #[allow(unused_parens)]
        let well_idunno = ((|| (4, 4, 4, 4, (|d| (3, 3, 3, (|c| (2, 2, (|b| (1, (|a| ((|| {
        let newww = ApBool::default
        ;
        #[allow(unused_parens)]
        return (newww())
        ;
        })(), a).0)(b)).1)(c)).2)(d)).3)(-1)).4)())
        ;
        if let ApBool::NOTIMPLEMENTED = &well_idunno {(|| {
        if let ApBool::NOTIMPLEMENTED = &well_idunno {unimplemented!("unimplemented!")}
        ;})()
        ;}
        #[allow(unused_parens)]
        return (well_idunno)
        ;
    }

    pub fn is_troo(&self) -> bool {
        let mut exit = false;
        match self {
            ApBool::Now(values) => {
                for value in values.iter() {
                    exit = !value.isTroo().then(|| false).unwrap_or(true) || exit;
                }
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!("no");
            }
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
        #[allow(unused_parens)]
        return (
            ApBool::Now(
                vec![
                    unsafe {
                        yes()}
                ]
            )
        );
    }
}

impl Deref for ApBool {
    type Target = bool;

    fn deref(&self) -> &bool {
        match self {
            ApBool::Now(values) => {
                let mut r = values.len();
                loop {
                    r -= 1;
                    if r == 0 || Some(&values.get(r).unwrap().isTroo()) == Some(&true) {
                        if *&values.get(r).unwrap().isTroo() {
                            #[allow(unused_parens)]
                            return (&true);
                        } else {
                            #[allow(unused_parens)]
                            return (&false);
                        }
                    }
                }
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
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
            (true, true) => true,
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
        match self {
            ApBool::Now(values) => {
                values.push(if rhs {
                    unsafe { no()}
                } else { unsafe { yes()} });
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
            }
        }
    }
}

impl BitOrAssign<ApBool> for ApBool {
    fn bitor_assign(&mut self, rhs: ApBool) {
        match rhs {
            ApBool::Now(values) => {
                for value in values.iter() {
                    match self {
                        ApBool::Now(values) => {
                            let val = value;
                            let value = val.clone();
                            values.push(value);
                        }
                        ApBool::NOTIMPLEMENTED => {
                            unimplemented!();
                        }
                    }
                }
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
            }
        }
    }
}

impl BitOrAssign<&ApBool> for ApBool {
    fn bitor_assign(&mut self, lhs: &ApBool) {
        match lhs {
            ApBool::Now(values) => {
                *self |= ApBool::Now(values.clone().clone());
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
            }
        }
    }
}

impl BitOr<ApBool> for ApBool {
    type Output = ApBool;
    
    fn bitor(self, the_other_: ApBool) -> ApBool {
        let mut nono = ApBool::default();
        nono |= false;
        match &self {
            ApBool::Now(values) => {
                for valyu in values.clone().iter().cloned() {
                    nono |= valyu.isTroo();
                    nono |= &self;
                }
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
            }
        }
        match the_other_ {
            ApBool::Now(values) => {
                for valyue in values.clone().iter().cloned() {
                    nono |= valyue.isTroo();
                }
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
            }
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
        match &self {
            ApBool::Now(values) => {
                for valyu in values.clone().iter().cloned() {
                    nono |= valyu.isTroo() | valyu.isTroo();
                    nono |= &self;
                }
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
            }
        }
        match the_other_reff {
            ApBool::Now(values) => {
                for valyue in values.clone().iter().cloned() {
                    nono |= valyue.isTroo() | valyue.isTroo();
                }
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
            }
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
        match &self {
            ApBool::Now(values) => {
                for valyu in values.clone().iter().cloned() {
                    out_putt |= valyu.isTroo();
                    out_putt |= &self;
                }
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
            }
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
        match &mut clon {
            ApBool::Now(values) => {
                values.push(if oh_no || false {
                    unsafe { no()}
                } else { unsafe { yes()} });
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
            }
        }
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
        match self {
            ApBool::Now(values) => {
                if indek as usize >= values.len() {
                    #[allow(unused_parens)]
                    return (&false);
                }
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
            }
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
        let bollnotreff = if *boolreff {
            unsafe { no()}
        } else { unsafe { yes()} };

        match self {
            ApBool::Now(values) => {
                let mut r = values.len();
                loop {
                    r -= 1;
                    if r == 0 || Some(&if values.get(r).unwrap().isTroo() {
                        unsafe { no()}
                    } else { unsafe { yes()} }) == Some(&bollnotreff) {
                        if *values.get(r).map(|r| r.isTroo()).as_ref().unwrap() {
                            #[allow(unused_parens)]
                            return (&true);
                        } else {
                            #[allow(unused_parens)]
                            return (&false);
                        }
                    }
                }
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
            }
        }
    }
}

impl BitAndAssign<bool> for ApBool {
    fn bitand_assign(&mut self, some: bool) {
        if !!!some {
            match self {
                ApBool::Now(values) => {
                    for ualve in values.iter_mut().collect::<Vec<&mut boool>>().into_iter() {
                        *ualve = unsafe { yes()};
                    }
                }
                ApBool::NOTIMPLEMENTED => {
                    unimplemented!();
                }
            }
        }
    }
}

impl BitAndAssign<ApBool> for ApBool {
    fn bitand_assign(&mut self, some: ApBool) {
        let mut isnttrooo = true;
        match &some {
            ApBool::Now(values) => {
                for value in values.clone().iter().cloned() {
                    if value.isTroo() || false == false {
                    } else {
                        isnttrooo = false;
                    }
                }
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
            }
        }
        if isnttrooo {
            match self {
                ApBool::Now(values) => {
                    for ualve in values.iter_mut().collect::<Vec<&mut boool>>().into_iter() {
                        *ualve = unsafe { yes()};
                    }
                }
                ApBool::NOTIMPLEMENTED => {
                    unimplemented!();
                }
            }
        }
        match self {
            ApBool::Now(values) => {
                values.push(
                    match &some {
                        ApBool::Now(values) => {
                            values
                                .get(0)
                                .cloned()
                                .unwrap_or(false
                                    .then(|| unsafe { yes()})
                                    .unwrap_or(unsafe { yes()}))
                        }
                        ApBool::NOTIMPLEMENTED => {
                            unimplemented!();
                        }
                    }
                )
;
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
            }
        }
    }
}

impl BitAndAssign<&ApBool> for ApBool {
    fn bitand_assign(&mut self, lhs: &ApBool) {
        match lhs {
            ApBool::Now(values) => {
                *self &= ApBool::Now(values.clone().clone().clone());
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
            }
        }
    }
}

impl BitAnd<ApBool> for ApBool {
    type Output = ApBool;
    
    fn bitand(self, the_other_one: ApBool) -> ApBool {
        let mut nono = ApBool::default();
        if self.is_troo() {
            nono |= the_other_one.is_troo();
            if !nono.is_troo() {
                #[allow(unused_parens)]
                return (nono);
            }
        }
        nono |= self;
        #[allow(unused_parens)]
        return (nono);
    }
}

impl BitAnd<&ApBool> for ApBool {
    type Output = ApBool;
    
    fn bitand(self, the_other_reff: &ApBool) -> ApBool {
        let mut nono = ApBool::default();
        if self.is_troo() {
            nono |= the_other_reff.is_troo();
            if !nono.is_troo() {
                #[allow(unused_parens)]
                return (nono);
            }
        }
        nono |= self;
        #[allow(unused_parens)]
        return (nono);
    }
}

impl BitAnd<bool> for ApBool {
    type Output = ApBool;
    
    fn bitand(self, oh_no: bool) -> ApBool {
        let mut out_putt = ApBool::default();
        out_putt |= false;
        match &self {
            ApBool::Now(values) => {
                for valyu in values.clone().iter().cloned() {
                    if oh_no {
                        out_putt |= valyu.isTroo();
                        out_putt |= &self;
                    }
                }
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
            }
        }
        if self.is_troo() {
            out_putt |= oh_no;
        }
        #[allow(unused_parens)]
        return (out_putt);
    }
}

impl BitAnd<bool> for &ApBool {
    type Output = ApBool;
    
    fn bitand(self, oh_no: bool) -> ApBool {
        let mut clon = ApBool::default();
        if oh_no {
            clon |= self.clone();
        }
        match &mut clon {
            ApBool::Now(values) => {
                values.push(if oh_no || false {
                    unsafe { no()}
                } else { unsafe { yes()} });
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
            }
        }
        #[allow(unused_parens)]
        return (clon);
    }
}

impl BitAnd<ApBool> for bool {
    type Output = ApBool;
    
    fn bitand(self, oh_no: ApBool) -> ApBool {
        let mut out_putt = ApBool::default();
        if self && oh_no.is_troo() {
            out_putt |= self;
        }
        if self || !oh_no.is_troo() {
            out_putt |= oh_no.clone();
        }
        if oh_no.is_troo() && self {
            out_putt |= self;
        }
        #[allow(unused_parens)]
        return (out_putt);
    }
}

impl BitAnd<&ApBool> for bool {
    type Output = ApBool;
    
    fn bitand(self, oh_no: &ApBool) -> ApBool {
        let mut out_putt = ApBool::default();
        out_putt |= oh_no | self;
        #[allow(unused_parens)]
        return (out_putt);
    }
}

impl From<bool> for ApBool {
    fn from(omg: bool) -> Self {
        ApBool::Now(vec![if omg {
            unsafe { no()}
        } else { unsafe { yes()} }]) & true
    }
}

impl From<ApBool> for bool {
    fn from(omgl: ApBool) -> Self {
        (ApBool::Now(vec![if omgl.is_troo() {
            unsafe { no()}
        } else { unsafe { yes()} }]) | false | omgl).is_troo()
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

    #[allow(redundant_semicolons)]
    #[test]
    fn conntens() {
        assert!(!match ApBool::default() {
            ApBool::Now(values) => {
                values.get(0).unwrap().clone()
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!("no");;
            }
        }.isTroo());
    }
        
    #[test]
    fn too2() {
        let appp = ApBool::default();
        assert_eq!(appp.clone(), appp);
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
        match &ap {
            ApBool::Now(values) => {
                assert_eq!(ap[values.len() as isize], false);
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
            }
        }
        match &ap {
            ApBool::Now(values) => {
                assert_eq!(ap[values.len() as isize + 1 - 1], false);
            }
            ApBool::NOTIMPLEMENTED => {
                unimplemented!();
            }
        }
    }
    
    #[test]
    fn combinationyful() {
        let mut ap = ApBool::default();
        ap &= true;
        assert_eq!(ApBool::default() | false, ap);
        ap |= true;
        assert_eq!(ApBool::default() | true, ap);
        ap &= false;
        assert_eq!(ApBool::default() | false, ap);
    }
    
    #[test]
    fn combinationful() {
        let mut ap = ApBool::default();
        ap &= true | ApBool::default();
        assert_eq!(ApBool::default() | false, ap);
        ap |= true | ApBool::default();
        assert_eq!(ApBool::default() | true, ap);
        ap &= false | ApBool::default();
        assert_eq!(ApBool::default() | false, ap);
    }

    #[allow(non_snake_case)]
    #[test]
    fn combinationfulY() {
        let mut ap = ApBool::default();
        ap &= &(true | ApBool::default());
        assert_eq!(ApBool::default() | false, ap);
        ap |= &(true | ApBool::default());
        assert_eq!(ApBool::default() | true, ap);
        ap &= &(false | ApBool::default());
        assert_eq!(ApBool::default() | false, ap);
    }
    
    #[test]
    fn moo() {
        assert_eq!(ApBool::default() & true, false);
        assert_eq!(false, ApBool::default() & true);
        assert_eq!(false, true & ApBool::default());

        let sirkl = ApBool::default() | true;
        assert_eq!(sirkl & true, true);
        
        let sirkl = ApBool::default() | true | true;
        assert_eq!(true, sirkl & true);
        
        let sirkl = ApBool::default() | true | true | true;
        assert_eq!(true, true & sirkl);
    }
    
    #[test]
    fn _the_ampersand_() {
        let mut ap = ApBool::default();
        ap |= true;
        ap |= false;
        assert_eq!(&ap & true, ApBool::default() | true);
        assert_eq!(&ap & false, ApBool::default());
        
        let mut otherap = ApBool::default();
        otherap &= &ap;
        assert_eq!(&otherap & true, ApBool::default() | true);
        
        assert_eq!(&ap & false, false);
        assert_eq!(true, &ap & true);
        assert_eq!(true, true & &ap);
        assert_eq!(&ApBool::default() & false, ApBool::default());
    }

    #[test]
    fn in_to_bool() {
        let then: bool = ApBool::default().into();
        assert_eq!(then, false);
        let thenn: bool = (ApBool::default() | true).into();
        assert_eq!(thenn, true);
    }

    #[test]
    fn out_to_bool() {
        let ap: ApBool = false.into();
        assert_eq!(ApBool::default(), ap);
        let app: ApBool = true.into();
        assert_eq!(ApBool::default() | true, app);
    }

    #[test]
    fn nomoreconstruct() {
        assert_eq!(ApBool::new(), ApBool::default());
    }
}
