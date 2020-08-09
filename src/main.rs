extern crate rand;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::io::{Error, ErrorKind};
use std::io::SeekFrom;
use std::str::FromStr;
use rand::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open(
        "./ammo.phantom.txt",
    )?;
    let mut reader = BufReader::new(file);

    let file_out = File::create(
        "./ammo.phantom.shuffle.txt",
    )?;
    let mut writer = BufWriter::new(file_out);

    let ammo_meta = read_ammo_meta(&mut reader)?;
    // println!("{:?}", ammo_meta);
    read_shuffle_write(&ammo_meta, &mut reader, &mut writer)?;
    writer.flush()?;
    Ok(())
}

fn read_ammo_meta(reader: &mut BufReader<File>) -> Result<Vec<(u64, u64)>, std::io::Error> {
    let mut ammo_meta = Vec::<(u64, u64)>::new();

    let mut offset: u64 = 0;
    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line)? as u64;
        if len == 0 {
            break;
        }
        let ammo_size = line.split(' ').next().and_then(|s| u64::from_str(s).ok());
        if let Some(ammo_size) = ammo_size {
            ammo_meta.push((offset, len + ammo_size));
            offset = reader.seek(std::io::SeekFrom::Start(offset + len + ammo_size))?;
        } else {
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }
    Ok(ammo_meta)
}

fn read_shuffle_write(
    meta: &Vec<(u64,u64)>,
    reader: &mut BufReader<File>,
    writer: &mut BufWriter<File>,
) -> Result<(), std::io::Error> {
    let mut rng = rand::thread_rng();
    let mut meta_shuffle = meta.clone();
    meta_shuffle.shuffle(&mut rng);

    for (offset, len) in meta_shuffle {
        reader.seek(SeekFrom::Start(offset))?;
        let mut buf = vec![0; len as usize];
        let ok = reader.read_exact(&mut buf);
        if ok.is_err() {
            println!("read incomplete at line (offset, len)={:?}, skipping", (offset, len));
            continue
        }
        writer.write(&mut buf)?;
    }
    Ok(())
}
