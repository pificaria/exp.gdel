fn tau(w: f64) -> f64 {
    96000. * (4000. * w.tanh()).sin()
}
