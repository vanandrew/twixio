use super::parsers;

#[derive(Debug)]
pub struct MrParcRaidHeader {
    pub header_id: u32,
    pub num_measurements: u32,
}

impl MrParcRaidHeader {
    pub fn new(input: &[u8]) -> Self {
        Self {
            header_id: parsers::header_id(input).unwrap().1,
            num_measurements: parsers::num_measurements(input).unwrap().1,
        }
    }
}

#[derive(Debug)]
pub struct MrParcRaidFileEntry {
    meas_id: u32,
    file_id: u32,
    measurement_offset: u64,
    measurement_length: u64,
    patient_name: String,
    protocol_name: String,
}

impl MrParcRaidFileEntry {
    pub fn new(input: &[u8]) -> Self {
        Self {
            meas_id: parsers::meas_id(input).unwrap().1,
            file_id: parsers::file_id(input).unwrap().1,
            measurement_offset: parsers::measurement_offset(input).unwrap().1,
            measurement_length: parsers::measurement_length(input).unwrap().1,
            patient_name: parsers::patient_name(input).unwrap().1,
            protocol_name: parsers::protocol_name(input).unwrap().1,
        }
    }
}
