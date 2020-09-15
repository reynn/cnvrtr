use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Zettagram {
    si: SiPrefix,
    n: f64,
}

impl Zettagram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Zetta,
            n,
        }
    }
}

impl Measurement for Zettagram {
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
        "Zettagram"
    }
}

impl Eq for Zettagram {}

impl From<Yottagram> for Zettagram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Zettagram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Zettagram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Zettagram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Zettagram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Zettagram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Zettagram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Zettagram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Zettagram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Zettagram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Zettagram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Zettagram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Zettagram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Zettagram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Zettagram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Zettagram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Zettagram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Zettagram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Zettagram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Zettagram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
