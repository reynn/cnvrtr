use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Kilogram {
    si: SiPrefix,
    n: f64,
}

impl Kilogram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Kilo,
            n,
        }
    }
}

impl Measurement for Kilogram {
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
        "Kilogram"
    }
}

impl Eq for Kilogram {}

impl From<Yottagram> for Kilogram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Kilogram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Kilogram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Kilogram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Kilogram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Kilogram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Kilogram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Kilogram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Kilogram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Kilogram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Kilogram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Kilogram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Kilogram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Kilogram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Kilogram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Kilogram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Kilogram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Kilogram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Kilogram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Kilogram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
