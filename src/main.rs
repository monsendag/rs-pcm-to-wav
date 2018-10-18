extern crate byteorder;
extern crate hound;

use std::env;
use std::fs::File;
use std::i16;
use std::io;
use std::io::prelude::*;
use std::io::Cursor;

use hound::WavSpec;
use hound::WavWriter;

use byteorder::{LittleEndian, ReadBytesExt};

/**
 * pass a list of files as arguments
 *
 * this program will iterate through all files
 * and add PCM wav headers, so the file can be
 * opened by a audio players
 */
fn main() {
    let args: Vec<String> = env::args().collect();

    // ignore first argument (name of binary)
    let args = &args[1..];

    for filename in args {
        match save(filename) {
            Ok(wav_name) => println!("saved filename as {}", wav_name),
            Err(_) => println!("failed to convert {} to wav file", filename),
        }
    }
}

/**
 * save as wav file, return new file name
*/
fn save(filename: &str) -> Result<String, io::Error> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut f = File::open(filename)?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer)?;

    let mut rdr = Cursor::new(&buffer);

    let mut value: Result<i16, io::Error>;

    let wav_name = format!("{}.wav", filename);
    let mut writer = WavWriter::create(&wav_name, spec).unwrap();

    loop {
        value = rdr.read_i16::<LittleEndian>();

        match value {
            Ok(x) => writer.write_sample(x).unwrap(),
            Err(_) => break,
        }
    }

    Ok(wav_name)
}
