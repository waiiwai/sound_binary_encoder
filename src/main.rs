use std::fs::File;
use std::io::{Read, BufReader};
use std::path::Path;
use std::env;

extern crate hound;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = File::open(&args[1]).expect("file not found");
    let path: &Path = &args[2].as_ref();

    let head_foot_f: f32 = 2048.0;
    let data_f: f32 = 256.0;

    let sample_rate = 32768.0;
    let sec_per_bit = 1.0; // 1秒あたりのbit数
    let bit_samples = (sample_rate/sec_per_bit) as i32; // 1bitのサンプル数
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = match path.is_file() {
        true => hound::WavWriter::append(path).unwrap(),
        false => hound::WavWriter::create(path, spec).unwrap(),
    };
    assert_eq!(spec, writer.spec());

    println!("Old duration is {} seconds.", writer.duration() / spec.sample_rate);

    let mut timer = 0.0;
    for _t in 0..bit_samples {
        let sample = square_wave(head_foot_f, timer);
        writer.write_sample(sample).unwrap();
        timer += 1.0/sample_rate;
    }

    for result in BufReader::new(input_file).bytes() {
        let byte = result.unwrap();

        for i in 0..8 {
            let f = if byte & 2_u8.pow(i) == 0 { 0.0 } else {data_f} ;
            let mut timer = 0.0;
            for _t in 0..bit_samples {
                let sample = square_wave(f, timer);
                writer.write_sample(sample).unwrap();
                timer += 1.0/sample_rate;
            }
        }
    }

    let mut timer = 0.0;
    for _t in 0..bit_samples {
        let sample = square_wave(head_foot_f, timer);
        writer.write_sample(sample).unwrap();
        timer += 1.0/sample_rate;
    }

    println!("New duration is {} seconds.", writer.duration() / spec.sample_rate);

    writer.finalize().unwrap();
}

fn square_wave(f: f32, t: f32) -> f32 {
    if f == 0.0 {
        0.0
    } else {
        let l = 1.0/f;
        let now = t % l;
        if now < l/2.0 {
            1.0
        } else {
            -1.0
        }
    }
}
