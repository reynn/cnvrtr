use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Gigagram {
    si: SiPrefix,
    n: f64,
}

impl Gigagram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Giga,
            n,
        }
    }
}

impl Measurement for Gigagram {
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
        "Gigagram"
    }
}

impl Eq for Gigagram {}

impl From<Yottagram> for Gigagram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Gigagram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Gigagram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Gigagram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Gigagram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Gigagram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Gigagram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Gigagram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Gigagram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Gigagram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Gigagram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Gigagram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Gigagram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Gigagram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Gigagram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Gigagram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Gigagram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Gigagram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Gigagram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Gigagram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
