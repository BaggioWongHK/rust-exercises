const FAHRENHEIT_CELSIUS_RATIO: f32 = 5.0/9.0;

pub fn temp_converter(input_unit: char, output_unit: char, value: f32) -> f32 {
    if input_unit == 'C' && output_unit == 'F' {
        value / FAHRENHEIT_CELSIUS_RATIO + 32.0
    } else if input_unit == 'F' && output_unit == 'C' {
        (value - 32.0) * FAHRENHEIT_CELSIUS_RATIO
    } else {
        value
    }
}