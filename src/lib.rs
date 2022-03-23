use std::ops::Deref;
use std::fmt::{Debug, Formatter, Result as FResult};

pub struct ApBool {
    values: Vec<bool>,
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
        self.values.get(0).unwrap()
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_thing() {
        let b: &bool = &ApBool::default();
        assert_eq!(b, &false);
    }
}
