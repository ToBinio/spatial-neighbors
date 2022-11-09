pub(crate) fn in_range(pos1: (f64, f64), pos2: (f64, f64), radius: f64) -> bool {
    (pos1.0 - pos2.0).powi(2) + (pos1.1 - pos2.1).powi(2) <= radius.powi(2)
}