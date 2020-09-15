use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Nanogram {
    si: SiPrefix,
    n: f64,
}

impl Nanogram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Nano,
            n,
        }
    }
}

impl Measurement for Nanogram {
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
        "Nanogram"
    }
}

impl Eq for Nanogram {}

impl From<Yottagram> for Nanogram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Nanogram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Nanogram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Nanogram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Nanogram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Nanogram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Nanogram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Nanogram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Nanogram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Nanogram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Nanogram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Nanogram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Nanogram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Nanogram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Nanogram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Nanogram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Nanogram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Nanogram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Nanogram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Nanogram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
