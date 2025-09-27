use dsmr_assignment::error::MainError;
use dsmr_assignment::runner::run;

fn main() -> Result<(), MainError> {
    #[cfg(test)]
    {
        run(true)
    }
    #[cfg(not(test))]
    {
        run(false)
    }
}
