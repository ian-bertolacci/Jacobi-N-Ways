extern crate argparse;
extern crate num;
extern crate core;
extern crate stopwatch;
extern crate rand;

use argparse::{ArgumentParser, StoreTrue, Store, StoreOption};
use stopwatch::{Stopwatch};
use std::default::{Default};
use num::{Num};
use core::clone::{Clone};
use std::time::{Duration};
use std::mem;
use std::thread;
use std::sync::{Arc, Mutex};
use rand::Rng;

fn main() {
  let mut _time_steps : Option<u32> = None;
  let mut _grid_size : Option<usize> = None;
  {  // this block limits scope of borrows by ap.refer() method
    let mut ap = ArgumentParser::new();
    // ap.set_description( "Greet somebody." );

    ap.refer( &mut _time_steps )
      .add_option( &["-T", "--time_steps"], StoreOption, "Number of iterations." );

    ap.refer( &mut _grid_size )
      .add_option( &["-N", "--grid_size"], StoreOption, "Edge-size of grid." );

    ap.parse_args_or_exit();
  }

  let time_steps = match _time_steps{
    Some(val) => val,
    None => 100
  };

  let grid_size = match _grid_size{
    Some(val) => val,
    None => 1000
  };

  println!( "N: {}\nT: {}", grid_size, time_steps );

  let mut grid_read = vec![0.0; grid_size+2];
  let mut grid_write = vec![0.0; grid_size+2];

  let mut rng = rand::thread_rng();
  for x in 1..grid_size {
    grid_read[x] = rng.gen::<f64>();
    grid_write[x] = grid_read[x];
  }

  let mut data_read = Arc::new( Mutex::new( grid_read ) );
  let mut data_write = Arc::new( Mutex::new( grid_write ) );

  let mut timer = Stopwatch::new();

  timer.start();

  for t in 1..time_steps{

    let mut children = vec![];

    for x in 1..grid_size {
      let data_read = data_read.clone();
      let data_write = data_write.clone();
      children.push(thread::spawn(move  || {
        let read = data_read.lock().unwrap();
        let mut write = data_write.lock().unwrap();
        write[x] = (read[x-1] + read[x] + read[x+1])*(1.0/3.0);
      }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }

    mem::swap(&mut data_read, &mut data_write);

  }

  timer.stop();

  println!("Elapsed: {}s", (timer.elapsed().num_nanoseconds().unwrap() as f64)/1e9 );

}
