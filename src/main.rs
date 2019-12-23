extern crate rand;
extern crate rodio;

use crate::rodio::Source;

use std::convert::TryInto;
use std::io::BufReader;

use rand::seq::SliceRandom;
use rand::Rng;

const WINDOW: usize = 64;

type RGen = rand::rngs::ThreadRng;

fn main() {
    let file = std::fs::File::open("samples/sample_short.mp3").unwrap();
    let samples = rodio::Decoder::new(BufReader::new(file)).unwrap();

    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);

    let source_sample_rate = samples.sample_rate();
    let source_channels = samples.channels();

    println!("Sample rate: {:?} Hz", source_sample_rate);

    // todo don't copy full track
    let mut audio: Vec<i16> = samples.into_iter().collect();
    let mut random = rand::thread_rng();

    let progress_len: u32 = (audio.len() / WINDOW).try_into().unwrap();
    let mut prev_progress = 0;

    for (n, slice) in audio.chunks_mut(WINDOW).enumerate() {
        mcall(&mut random, 1, |mut random| {
            shuffle::<i16>(slice, &mut random);
        });

        mcall(&mut random, 1, |mut random| {
            shuffle::<i16>(slice, &mut random);
        });

        prev_progress = print_progress(n, progress_len, prev_progress);
    }

    let buffer = rodio::buffer::SamplesBuffer::new(source_channels, source_sample_rate, audio);

    print!("\x1B[2J");
    println!("Playing. ^C to exit.");

    sink.append(buffer);
    sink.sleep_until_end();
}

fn shuffle<T>(chunk: &mut [T], random: &mut RGen) -> () {
    chunk.shuffle(random);
}

fn mcall(random: &mut RGen, threshold: u8, mut f: impl FnMut(&mut RGen) -> ()) -> () {
    if random.gen_range(0, 10) > threshold {
        f(random)
    }
}

#[allow(dead_code)]
fn retrigger<T: Copy>(_chunk: &mut [T], _random: &mut RGen) -> () {}

fn calc_progress (n: u32, full: u32) -> i8 {
  (((n + 1) as f32 / full as f32) * 100.) as i8
}

fn print_progress (n: usize, progress_len: u32, prev_progress: i8) -> i8{
  print!("\x1B[2J");
  match calc_progress(n as u32, progress_len) {
    p if p != prev_progress => {
      println!("Progress: {:?}%", calc_progress(n as u32, progress_len));
      return p;
    },
    _ => prev_progress,
  }
}
