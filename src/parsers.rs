use nom::{IResult, number};

fn take_u32(input: &[u8]) -> IResult<&[u8], u32> {
    number::complete::le_u32(input)
}

fn take_u64(input: &[u8]) -> IResult<&[u8], u64> {
    number::complete::le_u64(input)
}

// header id
pub fn header_id(input: &[u8]) -> IResult<&[u8], u32> {
    take_u32(&input[0..4])
}

// number of measurements
pub fn num_measurements(input: &[u8]) -> IResult<&[u8], u32> {
    take_u32(&input[4..8])
}

// meas id
pub fn meas_id(input: &[u8]) -> IResult<&[u8], u32> {
    take_u32(&input[0..4])
}

// file id
pub fn file_id(input: &[u8]) -> IResult<&[u8], u32> {
    take_u32(&input[4..8])
}

// measurement offset
pub fn measurement_offset(input: &[u8]) -> IResult<&[u8], u64> {
    take_u64(&input[8..16])
}

// measurement length
pub fn measurement_length(input: &[u8]) -> IResult<&[u8], u64> {
    take_u64(&input[16..24])
}

// patient name
pub fn patient_name(input: &[u8]) -> IResult<&[u8], String> {
    let (input, output) = nom::bytes::complete::take_until("\0")(&input[24..88])?;
    Ok((input, String::from_utf8_lossy(output).to_string()))
}

// protocol name
pub fn protocol_name(input: &[u8]) -> IResult<&[u8], String> {
    let (input, output) = nom::bytes::complete::take_until("\0")(&input[88..152])?;
    Ok((input, String::from_utf8_lossy(output).to_string()))
}
