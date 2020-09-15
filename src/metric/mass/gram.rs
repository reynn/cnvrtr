use super::*;
use crate::metric::prelude::*;

#[derive(Debug)]
pub struct Gram {
    si: SiPrefix,
    n: f64,
}

impl Gram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Base,
            n,
        }
    }
}

impl PartialEq for Gram {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl Measurement for Gram {
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
        "Gram"
    }
}

impl Eq for Gram {}

impl From<Yottagram> for Gram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Gram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Gram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Gram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Gram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Gram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Gram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Gram {
    fn from(kg: Kilogram) -> Self {
        Gram::new(kg.value() * kg.multiplier().unwrap())
    }
}
impl From<Hectogram> for Gram {
    fn from(hg: Hectogram) -> Self {
        Gram::new(hg.value() * hg.multiplier().unwrap())
    }
}
impl From<Decagram> for Gram {
    fn from(dg: Decagram) -> Self {
        Gram::new(dg.value() * dg.multiplier().unwrap())
    }
}
impl From<Decigram> for Gram {
    fn from(dg: Decigram) -> Self {
        Gram::new(dg.value() * dg.multiplier().unwrap())
    }
}
impl From<Centigram> for Gram {
    fn from(cg: Centigram) -> Self {
        Gram::new(cg.value() * cg.multiplier().unwrap())
    }
}
impl From<Milligram> for Gram {
    fn from(mg: Milligram) -> Self {
        Gram::new(mg.value() * mg.multiplier().unwrap())
    }
}
impl From<Microgram> for Gram {
    fn from(mg: Microgram) -> Self {
        Gram::new(mg.value() * mg.multiplier().unwrap())
    }
}
impl From<Nanogram> for Gram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Gram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Gram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Gram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Gram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Gram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
