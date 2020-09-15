use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Milligram {
    si: SiPrefix,
    n: f64,
}

impl Milligram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Milli,
            n,
        }
    }
}

impl Measurement for Milligram {
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
        "Milligram"
    }
}

impl Eq for Milligram {}

impl From<Yottagram> for Milligram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Milligram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Milligram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Milligram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Milligram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Milligram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Milligram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Milligram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Milligram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Milligram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Milligram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Milligram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Milligram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Milligram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Milligram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Milligram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Milligram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Milligram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Milligram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Milligram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
