use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Millimeter {
    si: SiPrefix,
    n: f64,
}

impl Millimeter {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Milli,
            n,
        }
    }
}

impl Measurement for Millimeter {
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
        "Millimeter"
    }
}

impl Eq for Millimeter {}

impl From<Meter> for Millimeter {
    fn from(m: Meter) -> Self {
        Millimeter::new(m.value() * m.multiplier().unwrap())
    }
}

impl From<Centimeter> for Millimeter {
    fn from(mm: Centimeter) -> Self {
        Millimeter::new(mm.value() * mm.multiplier().unwrap())
    }
}
