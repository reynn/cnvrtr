use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Foot {
    n: f64,
}

impl Foot {
    pub fn new(n: f64) -> Self {
        Self { n }
    }
}

impl Eq for Foot {}

impl Measurement for Foot {
    fn name(&self) -> &str {
        "Foot"
    }

    fn prefix(&self) -> Option<SiPrefix> {
        None
    }

    fn value(&self) -> f64 {
        self.n
    }

    fn update(&mut self, new: f64) -> Result<()> {
        Ok(self.n = new)
    }
}

/// Convert Meter to Feet
impl From<Meter> for Foot {
    fn from(m: Meter) -> Self {
        Foot::new(m.value() * 3.280839895)
    }
}

/// Convert Millimeter to Feet
impl From<Millimeter> for Foot {
    fn from(mm: Millimeter) -> Self {
        Foot::new(mm.value() / 304.8)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn m_to_f() {
//         let tests = vec![(100.0, 328.08398950000003), (43028.0, 141167.97900206002)];
//         for (val, expect) in tests {
//             let m = Meter::new(val);
//             let f: Foot = m.into();
//             assert_eq!(f, Foot(expect))
//         }
//     }

//     #[test]
//     fn mm_to_f() {
//         let tests = vec![(100.0, 0.32808398950131235), (43028.0, 141.16797900262466)];
//         for (val, expect) in tests {
//             let mm = Millimeter::new(val);
//             let f: Foot = mm.into();
//             assert_eq!(f, Foot(expect))
//         }
//     }
// }
