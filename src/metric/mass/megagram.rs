use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Megagram {
    si: SiPrefix,
    n: f64,
}

impl Megagram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Mega,
            n,
        }
    }
}

impl Measurement for Megagram {
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
        "Megagram"
    }
}

impl Eq for Megagram {}

impl From<Yottagram> for Megagram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Megagram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Megagram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Megagram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Megagram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Megagram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Megagram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Megagram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Megagram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Megagram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Megagram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Megagram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Megagram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Megagram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Megagram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Megagram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Megagram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Megagram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Megagram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Megagram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
