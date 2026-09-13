pub fn evaluate_risk_score(path_length: usize) -> u32 {
    if path_length <= 3 {
        95
    } else if path_length <= 6 {
        60
    } else {
        20
    }
}
