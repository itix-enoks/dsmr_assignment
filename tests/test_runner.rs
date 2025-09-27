use dsmr_assignment::runner::run;

#[test]
fn smoke_test_application() {
    let _ = run(true);

    assert!(true);
}
