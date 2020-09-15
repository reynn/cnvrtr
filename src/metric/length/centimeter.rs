use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Centimeter {
    si: SiPrefix,
    n: f64,
}

impl Centimeter {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Centi,
            n,
        }
    }
}

impl Measurement for Centimeter {
    fn name(&self) -> &str {
        "Centimeter"
    }

    fn value(&self) -> f64 {
        self.n
    }

    fn update(&mut self, new: f64) -> Result<()> {
        Ok(self.n = new)
    }

    fn prefix(&self) -> Option<SiPrefix> {
        Some(self.si.clone())
    }
}

impl Eq for Centimeter {}

impl From<Meter> for Centimeter {
    fn from(m: Meter) -> Self {
        if let Some(mult) = m.multiplier() {
            Centimeter::new(m.value() * mult)
        } else {
            Centimeter::new(m.value())
        }
    }
}

// impl From<Millimeter> for Centimeter {
//     fn from(mm: Millimeter) -> Self {
//         Centimeter(mm.0 * 1000 as f64)
//     }
// }
