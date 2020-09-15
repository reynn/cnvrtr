use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Exagram {
    si: SiPrefix,
    n: f64,
}

impl Exagram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Exa,
            n,
        }
    }
}

impl Measurement for Exagram {
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
        "Exagram"
    }
}

impl Eq for Exagram {}

impl From<Yottagram> for Exagram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Exagram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Exagram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Exagram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Exagram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Exagram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Exagram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Exagram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Exagram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Exagram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Exagram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Exagram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Exagram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Exagram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Exagram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Exagram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Exagram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Exagram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Exagram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Exagram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
