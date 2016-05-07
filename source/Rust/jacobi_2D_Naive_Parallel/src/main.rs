extern crate argparse;
extern crate num;
extern crate core;
extern crate stopwatch;
extern crate rand;
extern crate num_cpus;

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
use std::ops::Range;
use std::vec::Vec;
use std::cmp::{min,max};

struct Domain_Iter {
  current : Vec<i64>,
  domain : Domain
}

struct Domain {
  ranges : Vec<Range<i64>>,
  dim : usize,
}

struct Grid {
  domain : Domain,
  iter_domain : Domain,
  grid : Vec<f64>,

}

struct PartitionedGrid {
  full_domain : Domain,
  iter_domain : Domain,
  grids : Vec<Grid>,
}

impl PartitionedGrid {
  pub fn new_from_domain( domain : &Domain, iter_domain : &Domain, partitions : usize) -> PartitionedGrid {
    PartitionedGrid{
      full_domain : domain.clone(),
      iter_domain : iter_domain.clone(),
      grids : domain.split(partitions).iter().map( |_domain| { Grid::new_from_domain( _domain, &iter_domain.intersect( _domain )) } ).collect(),
    }
  }


  pub fn new_from_grid( grid : &Grid, partitions : usize ) -> PartitionedGrid {
    PartitionedGrid{
      full_domain : grid.domain.clone(),
      iter_domain : grid.iter_domain.clone(),
      grids : grid.domain.split(partitions).iter().map( |domain| { grid.subgrid(domain) } ).collect(),
    }
  }
  // */
}

impl Grid {
  pub fn new_from_domain( domain : &Domain, iter_domain : &Domain) -> Grid {
    Grid{
      domain : domain.clone(),
      iter_domain : iter_domain.clone(),
      grid : vec![0.0; domain.size() ]
    }
  }

  pub fn new_from_partitioned_grid( part_grid : &PartitionedGrid ) -> Grid {
    let mut grid = Grid::new_from_domain( &part_grid.full_domain, &part_grid.iter_domain );

    for subgrid in &part_grid.grids {
      for idx in subgrid.domain.iter(){
        grid.grid[ grid.domain.flat_idx( &idx[..] )] = subgrid.grid[subgrid.domain.flat_idx(&idx[..])];
      }
    }

    return grid;
  }

  pub fn subgrid( &self, subdomain : &Domain ) -> Grid {
    assert!( subdomain.is_subdomain(&self.domain) );

    if self.domain == *subdomain {
      return self.clone()
    }

    let mut subgrid = Grid::new_from_domain( &subdomain, &(self.iter_domain.intersect( subdomain)) );
    for idx in subdomain.iter() {
      subgrid.grid[ subgrid.domain.flat_idx( &idx[..] ) ] = self.grid[ self.domain.flat_idx( &idx[..] ) ];
    }

    return subgrid;
  }

}

impl Domain_Iter {
  fn new( domain : &Domain) -> Domain_Iter{
    let mut vec = (0..domain.dim).map(
      |idx| {
        domain.ranges[idx].start
      }
    ).collect::<Vec<i64>>();

    vec[domain.dim-1] -= 1;

    Domain_Iter{
      domain : domain.clone(),
      current : vec.clone(),
    }
  }
}

impl Iterator for Domain_Iter{
  type Item = Vec<i64>;

  fn next(&mut self) -> Option<Vec<i64>>{
    let mut bad = true;
    for pos in (0..self.domain.dim).rev() {
      self.current[pos] += 1;
      if self.current[pos] >= self.domain.ranges[pos].end {
        self.current[pos] = self.domain.ranges[pos].start;
      } else {
        bad = false;
        break;
      }
    }

    return if bad { None } else { Some( self.current.clone() ) };
  }

}

impl Clone for Grid {
  fn clone(&self) -> Self {
    Grid {
      domain : self.domain.clone(),
      iter_domain : self.iter_domain.clone(),
      grid : self.grid.clone(),
    }
  }
}

impl Domain {
  pub fn new_from_bounds(lower : &Vec<i64>, upper : &Vec<i64> ) -> Domain {
    assert!( upper.len() == lower.len() );
    assert!( upper.len() >= 1 );
    for i in 0..upper.len() {
      assert!( lower[i] <= upper[i] );
    }

    let ranges = (0..upper.len()).map(
        |dim| {
          Range{
            start: lower[dim],
            end: upper[dim]+1
          }
        }
      ).collect();

    Domain{
      dim : upper.len(),
      ranges : ranges,
    }
  }

  pub fn new_from_ranges( ranges : &Vec<Range<i64>> ) -> Domain {
    Domain{
      dim : ranges.len(),
      ranges : ranges.clone(),
    }
  }

  pub fn intersect( &self, other : &Domain ) -> Domain {
    assert!( self.dim == other.dim );

    Domain {
      dim : self.dim,
      ranges : (0..self.dim).map(
        |idx| {
          let my_range = self.ranges[idx].clone();
          let other_range = other.ranges[idx].clone();
          Range {
            start : max( my_range.start, other_range.start ),
            end : min( my_range.end, other_range.end ),
          }
      }).collect::<Vec<Range<_>>>(),
    }
  }

  fn iter(&self)->Domain_Iter{
    return Domain_Iter::new( self )
  }

  fn split( &self, partitions : usize ) -> Vec<Domain> {
    let extent = ((self.size_of_dim(0) as f64)/(partitions as f64)).ceil() as i64;
    let end = self.dim_range(0).end;

    let base = self.dim_range(0).start;

    return (0..partitions).map(
        |p| {
          let mut vec = vec![ Range{ start: base+(p as i64)*extent, end: min(base+(p as i64 +1)*extent,end)} ];
          vec.extend_from_slice( &self.ranges[1..self.dim] );
          Domain {
            dim : self.dim,
            ranges : vec,
          }
        }
      ).collect();
  }

  fn flat_idx( &self, args : &[i64] ) -> usize{
    assert!(args.len() == self.dim );

    let mut idx : i64 = 0;
    let mut mult : i64 = 1;

    for i in (0..self.dim).rev() {
      idx += (args[i] - self.ranges[i].start) * mult;
      mult *= self.ranges[i].clone().count() as i64;
    }

    return idx as usize;
  }

  fn size( &self ) -> usize {
    let mut mult : usize = 1;

    for i in 0..self.dim {
      mult *= self.size_of_dim(i);
    }

    return mult;
  }

  fn size_of_dim( &self, dim : usize ) -> usize{
    return self.ranges[dim].clone().count();
  }

  fn dim_range( &self, dim: usize )->Range<i64> {
    return self.ranges[dim].clone();
  }

  fn is_subdomain( &self, other: &Domain ) -> bool {
    return self.dim == other.dim
        && (0..self.dim).map(
            |idx| {
                 self.ranges[idx].start >= other.ranges[idx].start
              && self.ranges[idx].end <= other.ranges[idx].end
            }
          ).fold(true, |acc, x| { acc && x } );
  }

  fn is_strict_subdomain( &self, other: &Domain ) -> bool {
    return self.dim == other.dim
        && (0..self.dim).map(
            |idx| {
                 self.ranges[idx].start > other.ranges[idx].start
              && self.ranges[idx].end < other.ranges[idx].end
            }
          ).fold(true, |acc, x| { acc && x } );
  }
}

impl Clone for Domain {
  fn clone(&self) -> Self {
    Domain {
      dim : self.dim.clone(),
      ranges : self.ranges.clone(),
    }
  }
}

impl PartialEq for Domain {
  fn eq( &self, other: &Domain ) -> bool {
    if self.dim != other.dim {
      return false;
    }

    for i in 0..self.dim {
      if self.ranges[i] != other.ranges[i] {
        return false
      }
    }

    return true;
  }
}

fn wrap<T: Clone>( vec : &Vec<T> ) -> Vec<Arc<Mutex<T>>> {
  let mut ret = Vec::new();
  for idx in 0..vec.len() {
    ret.push( Arc::new(Mutex::new( vec[idx].clone() )) );
  }
  return ret;
}

fn unwrap<T: Clone>( vec : Vec<Arc<Mutex<T>>> ) -> Vec<T> {
  let mut ret = vec![];

  for value in vec {
    let value = value.clone();
    let value_ = value.lock().unwrap();
    ret.push( (*value_).clone() );
  }

  return ret;
}

fn main() {

  let mut _time_steps : Option<u32> = None;
  let mut _grid_size : Option<usize> = None;
  let mut _num_threads : Option<usize> = None;
  let mut verify : bool = false;

  {  // this block limits scope of borrows by ap.refer() method
    let mut ap = ArgumentParser::new();
    // ap.set_description( "Greet somebody." );

    ap.refer( &mut _time_steps )
      .add_option( &["-T", "--time_steps"], StoreOption, "Number of iterations." );

    ap.refer( &mut _grid_size )
      .add_option( &["-N", "--grid_size"], StoreOption, "Edge-size of grid." );

    ap.refer( &mut _num_threads )
      .add_option( &["-C", "--threads"], StoreOption, "Number of execution threads." );

    ap.refer( &mut verify )
      .add_option( &["-v", "--verify"], StoreTrue, "Verify the result against serial version." );

    ap.parse_args_or_exit();
  }

  let time_steps = match _time_steps{
    Some(val) => val,
    None => 2
  };

  let grid_size = match _grid_size{
    Some(val) => val,
    None => 4
  };

  let num_threads = match _num_threads{
    Some(val) => val,
    None => 4
  };

  println!( "N: {}\nT: {}", grid_size, time_steps );

  let grid_domain = Domain::new_from_ranges( &vec![ (0..grid_size as i64), (0..grid_size as i64) ] );
  let iter_domain = Domain::new_from_ranges( &vec![ (1..(grid_size as i64)-1), (1..(grid_size as i64)-1) ] );
  let mut read = Grid::new_from_domain( &grid_domain, &iter_domain );
  let mut write = Grid::new_from_domain( &grid_domain, &iter_domain );

  let mut i = 1.0;
  for idx in read.iter_domain.iter(){
    read.grid[ read.domain.flat_idx( &idx[..] ) ] = i;
    i += 1.0;
  }

  let mut timer = Stopwatch::new();

  timer.start();

  for t in 0..time_steps {
    let mut part_grid = PartitionedGrid::new_from_grid( &write, min( num_threads, grid_size-2 ) );
    let wrapped = wrap( &(part_grid.grids) );

    let mut children = vec![];

    for thread_id in 0..wrapped.len(){
      let write = wrapped[thread_id].clone();
      let read = read.clone();
      children.push( thread::spawn(
        move | | {
          let mut write = &mut *write.lock().unwrap();
          for idx in write.iter_domain.iter(){
            let x = idx[0];
            let y = idx[1];

            write.grid[ write.domain.flat_idx( &idx[..] ) ] =
              ( read.grid[ read.domain.flat_idx(&[x  ,y  ]) ]
              + read.grid[ read.domain.flat_idx(&[x-1,y  ]) ]
              + read.grid[ read.domain.flat_idx(&[x+1,y  ]) ]
              + read.grid[ read.domain.flat_idx(&[x  ,y-1]) ]
              + read.grid[ read.domain.flat_idx(&[x  ,y+1]) ] ) * 0.2;
          }
        } // thread
      ));
    } // for thread_id

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
    part_grid.grids = unwrap( wrapped );
    write = Grid::new_from_partitioned_grid( &part_grid );

    mem::swap(&mut read, &mut write);

  } // for t

  timer.stop();

  println!("Elapsed: {}s", (timer.elapsed().num_nanoseconds().unwrap() as f64)/1e9 );

  if verify {
    print!("Verifying: ");
    let mut v_read = Grid::new_from_domain( &grid_domain, &iter_domain );
    let mut v_write = Grid::new_from_domain( &grid_domain, &iter_domain );

    for t in 0..time_steps{
      for idx in v_write.iter_domain.iter(){
        let x = idx[0];
        let y = idx[1];
        v_write.grid[ v_write.domain.flat_idx( &[x,y] ) ] =
          ( v_read.grid[ v_read.domain.flat_idx( &[x  ,y  ] ) ]
          + v_read.grid[ v_read.domain.flat_idx( &[x-1,y  ] ) ]
          + v_read.grid[ v_read.domain.flat_idx( &[x+1,y  ] ) ]
          + v_read.grid[ v_read.domain.flat_idx( &[x  ,y-1] ) ]
          + v_read.grid[ v_read.domain.flat_idx( &[x  ,y+1] ) ] ) * 0.2;
      }
      mem::swap(&mut v_read, &mut v_write);
    }

    let verified = v_read.domain.iter().fold(true,
        |res, idx| {
          res &&
          v_read.grid[ v_read.domain.flat_idx(&idx[..]) ] == read.grid[ read.domain.flat_idx(&idx[..]) ]
        } );

    let mut failed = false;
    for idx in v_write.iter_domain.iter(){

      let v_val = v_read.grid[ v_read.domain.flat_idx(&idx[..]) ];
      let t_val = read.grid[ read.domain.flat_idx(&idx[..]) ];
      failed = (v_val == t_val);
      if( failed ){
        let x = idx[0];
        let y = idx[1];
        println!( "Failed! {} != {} @ ({}, {})", v_val, t_val, x, y );
      }
    }

    if( !failed ){
      println!("Passed!");
    }

  }
}
