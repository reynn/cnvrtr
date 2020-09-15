# CNVRTR

Converter is a library for unit conversion such as millimeters to inches, grams to lbs etc.

## Simple Examples

```rust
// Get a new Millimeter
let mm = Millimeter::new(170 as f64);
// Convert mm to inches
let in: Inch = mm.into();
assert_eq(in.value(), 6.692913);
```

## Supported Measurements

Length (Meters/Feet)
Mass (lbs/grams)
