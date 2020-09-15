use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Zeptogram {
    si: SiPrefix,
    n: f64,
}

impl Zeptogram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Zepto,
            n,
        }
    }
}

impl Measurement for Zeptogram {
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
        "Zeptogram"
    }
}

impl Eq for Zeptogram {}

impl From<Yottagram> for Zeptogram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Zeptogram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Zeptogram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Zeptogram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Zeptogram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Zeptogram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Zeptogram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Zeptogram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Zeptogram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Zeptogram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Zeptogram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Zeptogram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Zeptogram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Zeptogram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Zeptogram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Zeptogram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Zeptogram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Zeptogram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Zeptogram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Zeptogram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
