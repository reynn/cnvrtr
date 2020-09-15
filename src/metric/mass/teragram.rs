use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Teragram {
    si: SiPrefix,
    n: f64,
}

impl Teragram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Tera,
            n,
        }
    }
}

impl Measurement for Teragram {
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
        "Teragram"
    }
}

impl Eq for Teragram {}

impl From<Yottagram> for Teragram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Teragram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Teragram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Teragram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Teragram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Teragram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Teragram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Teragram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Teragram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Teragram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Teragram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Teragram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Teragram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Teragram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Teragram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Teragram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Teragram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Teragram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Teragram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Teragram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
