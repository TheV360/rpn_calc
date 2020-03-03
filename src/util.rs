pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
	(1.0 - t) * a + t * b
}
pub fn inv_lerp(a: f64, b: f64, n: f64) -> f64 {
	(n - a) / (b - a)
}
pub fn clamp(n: f64, min: f64, max: f64) -> f64 {
	n.max(min).min(max)
}
