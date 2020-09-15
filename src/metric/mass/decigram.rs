use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Decigram {
    si: SiPrefix,
    n: f64,
}

impl Decigram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Deci,
            n,
        }
    }
}

impl Measurement for Decigram {
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
        "Decigram"
    }
}

impl Eq for Decigram {}

impl From<Yottagram> for Decigram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Decigram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Decigram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Decigram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Decigram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Decigram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Decigram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Decigram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Decigram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Decigram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Decigram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Decigram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Decigram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Decigram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Decigram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Decigram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Decigram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Decigram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Decigram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Decigram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
