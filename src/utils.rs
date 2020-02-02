static ITER_UNITS: &[&str] = &["", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi", "Yi"];

pub fn convert_to_unit(mut count: f32) -> (f32, &'static str) {
    let mut suffix_index = 0;

    while count > 1024. && suffix_index + 1 < ITER_UNITS.len() {
        count /= 1024.;
        suffix_index += 1;
    }

    (count, ITER_UNITS[suffix_index])
}
