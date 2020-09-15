use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Microgram {
    si: SiPrefix,
    n: f64,
}

impl Microgram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Micro,
            n,
        }
    }
}

impl Measurement for Microgram {
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
        "Microgram"
    }
}

impl Eq for Microgram {}

impl From<Yottagram> for Microgram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Microgram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Microgram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Microgram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Microgram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Microgram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Microgram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Microgram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Microgram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Microgram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Microgram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Microgram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Microgram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Microgram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Microgram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Microgram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Microgram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Microgram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Microgram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Microgram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
