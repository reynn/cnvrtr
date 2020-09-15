pub mod length;
pub mod mass;
pub mod prelude;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum SiPrefix {
    Base,
    Yotta,
    Zetta,
    Exa,
    Peta,
    Tera,
    Giga,
    Mega,
    Kilo,
    Hecto,
    Deca,
    Deci,
    Centi,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto,
    Atto,
    Zepto,
    Yocto,
}

impl Default for SiPrefix {
    fn default() -> Self {
        SiPrefix::Base
    }
}

impl SiPrefix {
    pub fn parse(input_text: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match input_text.to_lowercase().as_str() {
            "yotta" => Ok(SiPrefix::Yotta),
            "zetta" => Ok(SiPrefix::Zetta),
            "exa" => Ok(SiPrefix::Exa),
            "peta" => Ok(SiPrefix::Peta),
            "tera" => Ok(SiPrefix::Tera),
            "giga" => Ok(SiPrefix::Giga),
            "mega" => Ok(SiPrefix::Mega),
            "kilo" => Ok(SiPrefix::Kilo),
            "hecto" => Ok(SiPrefix::Hecto),
            "deca" => Ok(SiPrefix::Deca),
            "base" => Ok(SiPrefix::Base),
            "deci" => Ok(SiPrefix::Deci),
            "centi" => Ok(SiPrefix::Centi),
            "milli" => Ok(SiPrefix::Milli),
            "micro" => Ok(SiPrefix::Micro),
            "nano" => Ok(SiPrefix::Nano),
            "pico" => Ok(SiPrefix::Pico),
            "femto" => Ok(SiPrefix::Femto),
            "atto" => Ok(SiPrefix::Atto),
            "zepto" => Ok(SiPrefix::Zepto),
            "yocto" => Ok(SiPrefix::Yocto),
            _ => Err("Failed to parse provided text".into()),
        }
    }

    pub fn get_multiplier(&self) -> Option<f64> {
        match self {
            SiPrefix::Yotta => Some(1000000000000000000000000.0),
            SiPrefix::Zetta => Some(1000000000000000000000.0),
            SiPrefix::Exa => Some(1000000000000000000.0),
            SiPrefix::Peta => Some(1000000000000000.0),
            SiPrefix::Tera => Some(1000000000000.0),
            SiPrefix::Giga => Some(1000000000.0),
            SiPrefix::Mega => Some(1000000.0),
            SiPrefix::Kilo => Some(1000.0),
            SiPrefix::Hecto => Some(100.0),
            SiPrefix::Deca => Some(10.0),
            SiPrefix::Base => Some(1.0),
            SiPrefix::Deci => Some(0.1),
            SiPrefix::Centi => Some(0.01),
            SiPrefix::Milli => Some(0.001),
            SiPrefix::Micro => Some(0.000001),
            SiPrefix::Nano => Some(0.000000001),
            SiPrefix::Pico => Some(0.000000000001),
            SiPrefix::Femto => Some(0.000000000000001),
            SiPrefix::Atto => Some(0.000000000000000001),
            SiPrefix::Zepto => Some(0.000000000000000000001),
            SiPrefix::Yocto => Some(0.000000000000000000000001),
        }
    }
}
