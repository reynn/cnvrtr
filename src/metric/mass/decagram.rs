use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Decagram {
    si: SiPrefix,
    n: f64,
}

impl Decagram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Deca,
            n,
        }
    }
}

impl Measurement for Decagram {
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
        "Decagram"
    }
}

impl Eq for Decagram {}

impl From<Yottagram> for Decagram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Decagram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Decagram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Decagram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Decagram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Decagram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Decagram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Decagram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Decagram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Gram> for Decagram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Decagram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Decagram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Decagram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Decagram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Decagram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Decagram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Decagram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Decagram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Decagram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Decagram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
