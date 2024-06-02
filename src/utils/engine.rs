pub fn eval_code(s: &str) -> String {
    use scryer_prolog::machine::mock_wam::*;

    let mut wam = Machine::with_test_streams();
    let bytes = wam.test_load_string(s);
    String::from_utf8_lossy(&bytes).to_string()
}
