use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Yottagram {
    si: SiPrefix,
    n: f64,
}

impl Yottagram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Yotta,
            n,
        }
    }
}

impl Measurement for Yottagram {
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
        "Yottagram"
    }
}

impl Eq for Yottagram {}

impl From<Zettagram> for Yottagram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Yottagram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Yottagram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Yottagram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Yottagram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Yottagram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Yottagram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Yottagram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Yottagram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Yottagram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Yottagram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Yottagram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Yottagram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Yottagram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Yottagram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Yottagram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Yottagram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Yottagram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Yottagram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Yottagram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
