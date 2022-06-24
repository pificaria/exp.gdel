fn tau(w: f64) -> f64 {
    1000. * ((500. * w).cos() * (30. * w).exp()).tanh().abs()
}
