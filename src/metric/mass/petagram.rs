use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Petagram {
    si: SiPrefix,
    n: f64,
}

impl Petagram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Peta,
            n,
        }
    }
}

impl Measurement for Petagram {
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
        "Petagram"
    }
}

impl Eq for Petagram {}

impl From<Yottagram> for Petagram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Petagram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Petagram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Petagram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Petagram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Petagram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Petagram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Petagram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Petagram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Petagram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Petagram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Petagram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Petagram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Petagram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Petagram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Petagram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Petagram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Petagram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Petagram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Petagram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
