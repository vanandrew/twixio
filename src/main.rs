use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;

mod parsers;
mod structures;
use structures::*;

fn main() -> Result<()> {
    // open the file
    let mut file = File::open(
        "/home/vanandrew/Data/a_ep_seg_fid_mdt_data/meas_MID00119_FID43267_a_ep_seg_fid_mdt.dat",
    )?;

    // read file into buffer
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // get the raid header
    let raid_header = MrParcRaidHeader::new(&buffer[0..])?;
    println!("{:#?}", raid_header);

    // for each measurement, get the measurement header
    for i in 0..raid_header.num_measurements {
        println!("\n\nMeasurement {}", i);
        let idx: usize = i.try_into()?;
        let file_entry = MrParcRaidFileEntry::new(&buffer[(8 + (idx * 152))..])?;
        println!("{:#?}", file_entry);
        let scan_start: usize = file_entry.measurement_offset.try_into()?;
        let scan_end: usize =
            (file_entry.measurement_offset + file_entry.measurement_length).try_into()?;
        let scan_entry = SingleMeasInit::new(&buffer[scan_start..scan_end])?;
        println!("{:#?}", scan_entry);
    }
    Ok(())
}
