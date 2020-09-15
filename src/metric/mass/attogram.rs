use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Attogram {
    si: SiPrefix,
    n: f64,
}

impl Attogram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Atto,
            n,
        }
    }
}

impl Measurement for Attogram {
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
        "Attogram"
    }
}

impl Eq for Attogram {}

impl From<Yottagram> for Attogram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Attogram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Attogram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Attogram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Attogram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Attogram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Attogram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Attogram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Attogram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Attogram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Attogram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Attogram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Attogram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Attogram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Attogram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Attogram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Attogram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Attogram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Attogram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Attogram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
