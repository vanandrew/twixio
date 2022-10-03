use super::parsers;
use anyhow::{anyhow, Error, Result};
use nom::{error, Finish};
use num::Unsigned;

fn s_to_v_err<T>(slice: error::Error<&[T]>) -> Error
where
    T: Unsigned + Clone + std::fmt::Debug,
{
    anyhow!("{:#?}", slice.input.to_vec())
}

#[derive(Debug)]
pub struct MrParcRaidHeader {
    pub header_id: u32,
    pub num_measurements: u32,
}

impl MrParcRaidHeader {
    pub fn new(input: &[u8]) -> Result<Self> {
        Ok(Self {
            header_id: parsers::header_id(input).finish().map_err(s_to_v_err)?.1,
            num_measurements: parsers::num_measurements(input)
                .finish()
                .map_err(s_to_v_err)?
                .1,
        })
    }
}

#[derive(Debug)]
pub struct MrParcRaidFileEntry {
    pub meas_id: u32,
    pub file_id: u32,
    pub measurement_offset: u64,
    pub measurement_length: u64,
    pub patient_name: String,
    pub protocol_name: String,
}

impl MrParcRaidFileEntry {
    pub fn new(input: &[u8]) -> Result<Self> {
        Ok(Self {
            meas_id: parsers::meas_id(input).finish().map_err(s_to_v_err)?.1,
            file_id: parsers::file_id(input).finish().map_err(s_to_v_err)?.1,
            measurement_offset: parsers::measurement_offset(input)
                .finish()
                .map_err(s_to_v_err)?
                .1,
            measurement_length: parsers::measurement_length(input)
                .finish()
                .map_err(s_to_v_err)?
                .1,
            patient_name: parsers::patient_name(input).finish().map_err(s_to_v_err)?.1,
            protocol_name: parsers::protocol_name(input)
                .finish()
                .map_err(s_to_v_err)?
                .1,
        })
    }
}

#[derive(Debug)]
pub struct SingleMeasInit {
    pub header_length: u32,
    pub unknown: u32,
}

impl SingleMeasInit {
    pub fn new(input: &[u8]) -> Result<Self> {
        Ok(Self {
            header_length: parsers::take_u32(&input[0..4])
                .finish()
                .map_err(s_to_v_err)?
                .1,
            unknown: parsers::take_u32(&input[4..8])
                .finish()
                .map_err(s_to_v_err)?
                .1,
        })
    }
}
