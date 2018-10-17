extern crate byteorder;
extern crate hound;

use std::env;
use std::f32::consts::PI;
use std::fs::File;
use std::i16;
use std::io;
use std::io::prelude::*;
use std::io::Cursor;

use hound::WavSpec;
use hound::WavWriter;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

fn main() {
    let args: Vec<String> = env::args().collect();

    for filename in &args[1..] {
        save(filename);
    }
}

/**
*
* 00000010
* 00000101

* 00000010
* 00010101

*/

fn save(filename: &str) -> Result<(), io::Error> {
    println!("save: {}", filename);
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut f = File::open(filename)?;
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer)?;

    let mut rdr = Cursor::new(&buffer);

    let mut value: Result<i16, io::Error>;

    let wav_name = format!("{}.wav", filename);
    let mut writer = WavWriter::create(wav_name, spec).unwrap();

    loop {
        value = rdr.read_i16::<LittleEndian>();

        match (value) {
            Ok(x) => writer.write_sample(x).unwrap(),
            Err(_) => break,
        }
    }

    Ok(())
}
