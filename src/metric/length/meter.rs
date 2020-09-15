use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Meter {
    si: SiPrefix,
    n: f64,
}

impl Eq for Meter {}

impl Meter {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Base,
            n,
        }
    }
}

impl Measurement for Meter {
    fn value(&self) -> f64 {
        self.n
    }

    fn update(&mut self, new: f64) -> Result<()> {
        Ok(self.n = new)
    }

    fn prefix(&self) -> Option<SiPrefix> {
        Some(self.si.clone())
    }

    fn name(&self) -> &str {
        "Meter"
    }
}

impl From<Centimeter> for Meter {
    fn from(cm: Centimeter) -> Self {
        Meter::new(cm.value() * cm.multiplier().unwrap())
    }
}

impl From<Millimeter> for Meter {
    fn from(mm: Millimeter) -> Self {
        Meter::new(mm.value() * mm.multiplier().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mm_to_m() {
        let tests = vec![(100.0, 0.1), (43028.0, 43.028)];
        for (val, expect) in tests {
            let mm = Millimeter::new(val);
            let m: Meter = mm.into();
            assert_eq!(m, Meter::new(expect))
        }
    }

    #[test]
    fn cm_to_m() {
        let tests = vec![(100.0, 1.0), (43028.0, 430.28000000000003)];
        for (val, expect) in tests {
            let cm = Centimeter::new(val);
            let m: Meter = cm.into();
            assert_eq!(m, Meter::new(expect))
        }
    }
}
