use crate::metric::prelude::*;

pub trait Measurement {
    fn name(&self) -> &str;
    fn prefix(&self) -> Option<SiPrefix>;
    fn multiplier(&self) -> Option<f64> {
        if let Some(prefix) = self.prefix() {
            prefix.get_multiplier()
        } else {
            None
        }
    }
    fn value(&self) -> f64;
    fn update(&mut self, new: f64) -> Result<()>;
}
