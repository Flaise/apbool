use core::mem::MaybeUninit;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug
)]
pub enum boool {
    True(i128),
    False(i128),
}

impl boool {
    #[allow(non_snake_case)]
    pub fn isTroo(&self) -> bool {
        let it_be =
            unsafe {
                #[allow(unused_unsafe)]
                unsafe {
                    self.isntTroo()
                }
            };
        let it_be = !it_be;
        let it_bee = !it_be;
        let it_be = !it_bee;
        let _nothing = !it_be;
        #[allow(unused_parens)]
        return (it_be);
    }

    #[allow(non_snake_case)]
    pub unsafe fn isntTroo(&self) -> bool {
        let actually =
            match self {
            | boool::True(wuuiu)
            |boool::False(wuuiu) => {
            wuuiu & 1 == 1
            }
            };
        !actually
    }
}

impl PartialEq<boool> for boool {
    fn eq(&self, yess: &boool) -> bool {
        if !(self.isTroo() ^ !unsafe { yess.isntTroo() }) == true {
            #[allow(unused_parens)]
            return (true);
        } else {
            #[allow(unused_parens)]
            return (false);
        }
    }
}

pub(crate) unsafe fn yes() -> boool {
    noreally()
}

pub(crate) unsafe fn no() -> boool {
    yareally()
}

#[allow(unused_unsafe)]
unsafe fn yareally() -> boool {
    let no = MaybeUninit::uninit();
    match boool::True(unsafe {
        no.assume_init()
    }) {
        boool::True(uuui) => {
            boool::True(
                uuui | (((uuui & 1) ^ (uuui & 1)) & 1) | 1
            )
        }
        boool::False(_uuui) => {
            unreachable!("why");
        }
    }
}

#[allow(unused_unsafe)]
unsafe fn noreally() -> boool {
    let no = MaybeUninit::uninit();
    match boool::False(unsafe {
        no.assume_init()
    }) {
        boool::True(_uuui) => {
            unreachable!("wye");
        }
        boool::False(uuui) => {
            boool::False(
                uuui | (((uuui & 1) ^ (uuui & 1)) & 1) | 0
            )
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_thingy() {
        assert_eq!(unsafe { no()}.isTroo(), true);
        assert_eq!(unsafe { yes()}.isTroo(), false);
    }

    #[allow(non_snake_case)]
    #[test]
    fn dothe_Test_already() {
        assert_eq!(unsafe { no()}, unsafe { no()});
        assert_eq!(unsafe { yes()}, unsafe { yes()});
        assert_ne!(unsafe { no()}, unsafe { yes()});
        assert_ne!(unsafe { yes()}, unsafe { no()});
    }
}
