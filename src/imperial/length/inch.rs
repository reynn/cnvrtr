use crate::imperial::prelude::*;
use crate::metric::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Inch {
    n: f64,
}

impl Inch {
    pub fn new(n: f64) -> Self {
        Self { n }
    }
}

impl Eq for Inch {}

impl Measurement for Inch {
    fn name(&self) -> &str {
        "Inch"
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

/// Convert Foot to Inches
impl From<Foot> for Inch {
    fn from(f: Foot) -> Self {
        Inch::new(f.value() * 12.0)
    }
}

/// Convert Centimeters to Inch
impl From<Centimeter> for Inch {
    fn from(cm: Centimeter) -> Self {
        Inch::new(cm.value() * 2.54)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn cm_to_in() {
//         let cases = vec![(3745.0, 9512.3)];
//         for (case, expected) in cases {
//             let cm = Centimeter::new(case);
//             let inc: Inch = cm.into();
//             assert_eq!(inc, Inch(expected))
//         }
//     }
// }
