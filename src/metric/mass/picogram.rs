use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Picogram {
    si: SiPrefix,
    n: f64,
}

impl Picogram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Pico,
            n,
        }
    }
}

impl Measurement for Picogram {
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
        "Picogram"
    }
}

impl Eq for Picogram {}

impl From<Yottagram> for Picogram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Picogram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Picogram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Picogram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Picogram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Picogram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Picogram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Picogram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Picogram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Picogram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Picogram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Picogram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Picogram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Picogram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Picogram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Picogram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Picogram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Picogram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Picogram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
impl From<Yoctogram> for Picogram {
    fn from(_: Yoctogram) -> Self {
        todo!()
    }
}
