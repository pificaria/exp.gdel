fn tau(w: f64) -> f64 {
    if w < 0.005 {
        0.
        //14000. * (8. * (40. * w).sin()).tanh()
    } else if w < 0.05 {
        24000. * (1000. * (1. - w).sin()).cos().abs()
    } else {
        12000.*w
    }
}
