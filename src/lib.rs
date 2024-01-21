// Cristian Echeverría
// REQUIERE MUCHO TRABAJO
// PARECE QUE ESTE ESTILO NO ES COMPATIBLE CON RUST

use std::{cmp::PartialOrd, fmt::Display};

//----------------------------------------------------------------------------------------

pub type CheckResult = Result<(), String>;

//----------------------------------------------------------------------------------------

pub struct Check<T> {
    pub n: T
}

impl<T: PartialOrd + Display> Check<T> {

    pub fn lt(&self, v:T, msg: &str) -> CheckResult {
        if self.n >= v {
            return Err(format!("{msg} [ {} >= {v} ]", self.n));
        }
        Ok(())
    }
    
    pub fn le(&self, v:T, msg: &str) -> CheckResult {
        if self.n > v {
            return Err(format!("{msg} [ {} > {v} ]", self.n));
        }
        Ok(())
    }

    pub fn gt(&self, v:T, msg: &str) -> CheckResult {
        if self.n <= v {
            return Err(format!("{msg} [ {} <= {v} ]", self.n));
        }
        Ok(())
    }

    pub fn ge(&self, v:T, msg: &str) -> CheckResult {
        if self.n < v {
            return Err(format!("{msg} [ {} < {v}]", self.n));
        }
        Ok(())
    }

}


//----------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // use crate as check;
    use super::*;

    #[test]
    fn lt_ok() {
        if let Err(e) = (Check{n:3.0}).lt(4.0, "") { panic!("Error: {}", e); }
        assert!(true);
    }

    #[test]
    fn lt_err() {
        let x = Check{n:5.0}.lt(4.0, "");
        assert!(x.is_err());
    }

    #[test]
    fn le_ok() {
        if let Err(e) = (Check{n:3.0}).le(3.0, "") { panic!("Error: {}", e); }
        assert!(true);
    }

    #[test]
    fn le_err() {
        let x = Check{n:3.0}.le(2.99, "");
        assert!(x.is_err());
    }

}
