static ITER_UNITS: &[&str] = &["", "K", "M", "G", "T", "P", "E", "Z", "Y"];

pub fn convert_to_unit(mut count: f32) -> (f32, &'static str) {
    let mut suffix_index = 0;

    while count > 1000. && suffix_index + 1 < ITER_UNITS.len() {
        count /= 1000.;
        suffix_index += 1;
    }

    (count, ITER_UNITS[suffix_index])
}
