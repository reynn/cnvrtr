use super::*;
use crate::metric::prelude::*;

#[derive(Debug)]
pub struct Hectogram {
    si: SiPrefix,
    n: f64,
}

impl PartialEq for Hectogram {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl Hectogram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Hecto,
            n,
        }
    }
}

impl Measurement for Hectogram {
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
        "Hectogram"
    }
}

impl Eq for Hectogram {}

impl From<Yottagram> for Hectogram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Hectogram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Hectogram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Hectogram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Hectogram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Hectogram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Hectogram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Hectogram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Hectogram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Hectogram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Hectogram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Hectogram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Hectogram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Hectogram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Hectogram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Hectogram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Hectogram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Hectogram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Hectogram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Hectogram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
