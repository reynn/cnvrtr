pub mod attogram;
pub mod centigram;
pub mod decagram;
pub mod decigram;
pub mod exagram;
pub mod femtogram;
pub mod gigagram;
pub mod gram;
pub mod hectogram;
pub mod kilogram;
pub mod megagram;
pub mod microgram;
pub mod milligram;
pub mod nanogram;
pub mod petagram;
pub mod picogram;
pub mod teragram;
pub mod yoctogram;
pub mod yottagram;
pub mod zeptogram;
pub mod zettagram;

pub use attogram::*;
pub use centigram::*;
pub use decagram::*;
pub use decigram::*;
pub use exagram::*;
pub use femtogram::*;
pub use gigagram::*;
pub use gram::*;
pub use hectogram::*;
pub use kilogram::*;
pub use megagram::*;
pub use microgram::*;
pub use milligram::*;
pub use nanogram::*;
pub use petagram::*;
pub use picogram::*;
pub use teragram::*;
pub use yoctogram::*;
pub use yottagram::*;
pub use zeptogram::*;
pub use zettagram::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grams_to_milligrams() {
        let test_cases = vec![
            (123.18, 123180.0),
            (382.0, 382000.0),
            (4723.0, 4723000.0),
            (8302.99, 8302990.0),
            (18201.1, 18201100.0),
            (1276756.12354, 1276756123.54),
        ];
        for (tc, expected) in test_cases {
            let g = Gram::new(tc);
            let mg: Milligram = g.into();
            assert_eq!(mg, Milligram::new(expected));
        }
    }
    #[test]
    fn milligrams_to_grams() {
        let test_cases = vec![
            (123180.0, 123.18),
            (382000.0, 382.0),
            (4723000.0, 4723.0),
            (8302990.0, 8302.99),
            (18201100.0, 18201.1),
            (1276756123.54, 1276756.12354),
        ];
        for (tc, expected) in test_cases {
            let mg = Milligram::new(tc);
            let g: Gram = mg.into();
            assert_eq!(g, Gram::new(expected));
        }
    }
    #[test]
    fn grams_to_micrograms() {
        let test_cases = vec![
            (123.18, 123180000.0),
            (382.0, 382000000.0),
            (4723.0, 4723000000.0),
            (8302.99, 8302990000.0),
            (18201.1, 18201100000.0),
            (1276756.12354, 1276756123540.0),
        ];
        for (tc, expected) in test_cases {
            let g = Gram::new(tc);
            let mg: Microgram = g.into();
            assert_eq!(mg, Microgram::new(expected));
        }
    }
    #[test]
    fn micrograms_to_grams() {
        let test_cases = vec![
            (123180000.0, 123.17999999999999),
            (382000000.0, 382.0),
            (4723000000.0, 4723.0),
            (8302990000.0, 8302.99),
            (18201100000.0, 18201.1),
            (1276756123540.0, 1276756.1235399998),
        ];
        for (tc, expected) in test_cases {
            let mg = Microgram::new(tc);
            let g: Gram = mg.into();
            assert_eq!(g, Gram::new(expected));
        }
    }
}
