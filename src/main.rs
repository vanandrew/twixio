use std::io::prelude::*;
use std::io::Result;
use std::fs::File;

mod parsers;
mod structures;
use structures::*;

fn main() -> Result<()> {
    // open the file
    let mut file = File::open("/home/vanandrew/Data/a_ep_seg_fid_mdt_data/meas_MID00119_FID43267_a_ep_seg_fid_mdt.dat")?;
    
    // read file into buffer
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let raid_header = MrParcRaidHeader::new(&buffer[0..]);
    println!("{:?}", raid_header);
    for i in 0..raid_header.num_measurements {
        let idx: usize = i.try_into().unwrap();
        let file_entry = MrParcRaidFileEntry::new(&buffer[(8 + (idx * 152))..]);
        println!("{:?}", file_entry);
    }
    Ok(())
}

