use std::fs;

use crate::bail;

use tudelft_dsmr_output_generator::voltage_over_time::create_voltage_over_time_graph;
use tudelft_dsmr_output_generator::Graphs;

use crate::telegram::Value;
use crate::error::MainError;
use crate::parser::parse;

use crate::helpers::*;

pub fn run(test: bool) -> Result<(), MainError> {
    let input: &str = if !test {
        &read_from_stdin()?
    } else {
        &fs::read_to_string("examples/good/two_packets.dsmr")
        .expect("Failed to read two_packets.dsmr file")

    };

    let telegrams = parse(&input);
    let mut telegrams = telegrams.unwrap_or_else(|_| bail!("Failed parsing telegram(s)"));

    telegrams.sort_by_key(|t| match &t.base.date.value {
        Some(Value::Date(date)) => date.timestamp,
        _ => bail!("Invalid timestamp")
    });
    let telegrams = telegrams; // We can by now assume that telegrams are always sorted by date

    let voltages = process_voltages(&telegrams);
    let current_over_time = process_currents(&telegrams);
    let gas_delta_over_time = process_gas_data(&telegrams);
    let energy_pair_delta_over_time = process_energy_data(&telegrams);

    let mut result = Graphs::new()?;
    process_event_logs(&telegrams, &mut result);

    result.add_graph(create_voltage_over_time_graph(voltages))?;
    result.add_graph(current_over_time)?;
    result.add_graph(gas_delta_over_time)?;
    result.add_graph(energy_pair_delta_over_time)?;
    let _ = result.generate();

    Ok(())
}
