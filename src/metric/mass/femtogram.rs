use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Femtogram {
    si: SiPrefix,
    n: f64,
}

impl Femtogram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Femto,
            n,
        }
    }
}

impl Measurement for Femtogram {
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
        "Femtogram"
    }
}

impl Eq for Femtogram {}

impl From<Yottagram> for Femtogram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Femtogram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Femtogram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Femtogram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Femtogram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Femtogram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Femtogram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Femtogram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Femtogram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Femtogram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Femtogram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Femtogram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Femtogram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Femtogram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Femtogram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Femtogram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Femtogram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Femtogram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Femtogram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Femtogram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
