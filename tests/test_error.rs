use tudelft_dsmr_output_generator::PlotError;

use dsmr_assignment::error::MainError;

#[test]
fn test_main_error_from_io_error() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let main_err: MainError = io_err.into();

    match main_err {
        MainError::IoError(_) => assert!(true),
        _ => panic!("Expected IoError variant"),
    }
}

#[test]
fn test_main_error_from_plot_error() {
    let err = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let plot_err = PlotError::Io(err);
    let main_err: MainError = plot_err.into();

    match main_err {
        MainError::PlotError(_) => assert!(true),
        _ => panic!("Expected PlotError variant"),
    }
}
