use super::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Yoctogram {
    si: SiPrefix,
    n: f64,
}

impl Yoctogram {
    pub fn new(n: f64) -> Self {
        Self {
            si: SiPrefix::Yocto,
            n,
        }
    }
}

impl Measurement for Yoctogram {
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
        "Yoctogram"
    }
}

impl Eq for Yoctogram {}

impl From<Yottagram> for Yoctogram {
    fn from(_: Yottagram) -> Self {
        todo!()
    }
}
impl From<Zettagram> for Yoctogram {
    fn from(_: Zettagram) -> Self {
        todo!()
    }
}
impl From<Petagram> for Yoctogram {
    fn from(_: Petagram) -> Self {
        todo!()
    }
}
impl From<Teragram> for Yoctogram {
    fn from(_: Teragram) -> Self {
        todo!()
    }
}
impl From<Gigagram> for Yoctogram {
    fn from(_: Gigagram) -> Self {
        todo!()
    }
}
impl From<Megagram> for Yoctogram {
    fn from(_: Megagram) -> Self {
        todo!()
    }
}
impl From<Exagram> for Yoctogram {
    fn from(_: Exagram) -> Self {
        todo!()
    }
}
impl From<Kilogram> for Yoctogram {
    fn from(_: Kilogram) -> Self {
        todo!()
    }
}
impl From<Hectogram> for Yoctogram {
    fn from(_: Hectogram) -> Self {
        todo!()
    }
}
impl From<Decagram> for Yoctogram {
    fn from(_: Decagram) -> Self {
        todo!()
    }
}
impl From<Gram> for Yoctogram {
    fn from(_: Gram) -> Self {
        todo!()
    }
}
impl From<Decigram> for Yoctogram {
    fn from(_: Decigram) -> Self {
        todo!()
    }
}
impl From<Centigram> for Yoctogram {
    fn from(_: Centigram) -> Self {
        todo!()
    }
}
impl From<Milligram> for Yoctogram {
    fn from(_: Milligram) -> Self {
        todo!()
    }
}
impl From<Microgram> for Yoctogram {
    fn from(_: Microgram) -> Self {
        todo!()
    }
}
impl From<Nanogram> for Yoctogram {
    fn from(_: Nanogram) -> Self {
        todo!()
    }
}
impl From<Picogram> for Yoctogram {
    fn from(_: Picogram) -> Self {
        todo!()
    }
}
impl From<Femtogram> for Yoctogram {
    fn from(_: Femtogram) -> Self {
        todo!()
    }
}
impl From<Attogram> for Yoctogram {
    fn from(_: Attogram) -> Self {
        todo!()
    }
}
impl From<Zeptogram> for Yoctogram {
    fn from(_: Zeptogram) -> Self {
        todo!()
    }
}
