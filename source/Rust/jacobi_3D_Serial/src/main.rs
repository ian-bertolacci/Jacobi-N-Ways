extern crate argparse;
extern crate num;
extern crate core;
extern crate stopwatch;

use argparse::{ArgumentParser, StoreTrue, Store, StoreOption};
use stopwatch::{Stopwatch};
use std::default::{Default};
use num::{Num};
use core::clone::{Clone};
use std::time::{Duration};


struct Grid<T: Num + Default + Clone> {
  size : usize,
  grid : Vec<T>,
}

impl<T: Num + Default + Clone> Grid<T> {
  pub fn new( _size : usize ) -> Grid<T>{
    Grid{
      size: _size as usize,
      grid: vec![ Default::default(); _size*_size*_size as usize ]
    }
  }

  pub fn read(&self,  x: usize, y: usize, z: usize ) -> &T {
    return &self.grid[x + self.size*y + self.size*self.size*z];
  }

  pub fn write(&mut self,  x: usize, y: usize, z: usize, value: T ) {
    self.grid[x + self.size*y + self.size*self.size*z] = value;
  }

}

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

  let mut grid_a : Grid<i32> = Grid::new( grid_size );
  let mut grid_b : Grid<i32> = Grid::new( grid_size );

  let mut timer = Stopwatch::new();

  timer.start();

  for t in 1..time_steps{
    if t & 1 == 0 {
      for x in 1..grid_b.size-2 {
        for y in 1..grid_b.size-2 {
          for z in 1..grid_b.size-2 {
            let next = (     grid_a.read(x,y,z-1) +
                                              grid_a.read(x,y-1,z) +
                       grid_a.read(x-1,y,z) + grid_a.read(x,y  ,z) + grid_a.read(x+1,y,z) +
                                              grid_a.read(x,y+1,z) +
                                                              grid_a.read(x,y,z+1)
                        ) / 7;
            grid_b.write(x,y,z, next);
          }
        }
      }
    } else {
      for x in 1..grid_b.size-2 {
        for y in 1..grid_b.size-2 {
          for z in 1..grid_b.size-2 {
            let next = (     grid_b.read(x,y,z-1) +
                                              grid_b.read(x,y-1,z) +
                       grid_b.read(x-1,y,z) + grid_b.read(x,y  ,z) + grid_b.read(x+1,y,z) +
                                              grid_b.read(x,y+1,z) +
                                                              grid_b.read(x,y,z+1)
                        ) / 7;
            grid_a.write(x,y,z, next);
          }
        }
      }
    }
  }

  timer.stop();

  println!("Elapsed: {}s", (timer.elapsed().num_nanoseconds().unwrap() as f64)/1e9 );

}
