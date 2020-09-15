use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Centigram {
    si: SiPrefix,
    n: f64,
}

impl Centigram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Centi,
            n,
        }
    }
}

impl Measurement for Centigram {
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
        "Centigram"
    }
}

impl Eq for Centigram {}

impl From<Yottagram> for Centigram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Centigram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Centigram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Centigram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Centigram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Centigram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Centigram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Centigram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Centigram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Centigram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Centigram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Centigram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Centigram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Centigram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Centigram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Centigram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Centigram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Centigram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Centigram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Centigram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
