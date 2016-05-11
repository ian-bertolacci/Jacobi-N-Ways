use Time;

config const N = 1000;
config const T = 100;
config const verify : bool;
config const epsilon = 0.000001;

proc main(){

  var iteration_domain: domain(3) = { 1..#N, 1..#N, 1..#N };
  var grid_domain : domain(4) = { 0..1, 0..#N+2, 0..#N+2, 0..#N+2 };

  writeln( "N: " + N );
  writeln( "T: " + T );

  var grid : [grid_domain] real(64);

  var value = 1.0;
  for (x,y,z) in iteration_domain {
    grid[0,x,y,z] = value;
    grid[1,x,y,z] = value;
    value += 1;
  }

  var timer : Timer;

  timer.start();

  for t in 1..T {
    var read = t & 1;
    var write = 1 - read;
    forall (x,y,z) in iteration_domain{
      grid[write,x,y,z] = ( grid[read,x,y,z] +
                            grid[read,x+1,y,z] + grid[read,x,y+1,z] + grid[read,x,y,z+1] +
                            grid[read,x-1,y,z] + grid[read,x,y-1,z] + grid[read,x,y,z-1]
                          ) * (1/7);
    }
  }

  timer.stop();

  writeln( "Elapsed: " + timer.elapsed() + "s" );

  if verify {
    writeln("Verifying: ");
    var v_grid : [grid_domain] real(64);

    value = 1.0;
    for (x,y,z) in iteration_domain {
      v_grid[0,x,y,z] = value;
      v_grid[1,x,y,z] = value;
      value += 1;
    }

    for t in 1..T {
      var read = t & 1;
      var write = 1 - read;
      for (x,y,z) in iteration_domain{
        v_grid[write,x,y,z] = ( v_grid[read,x,y,z] +
                                v_grid[read,x+1,y,z] + v_grid[read,x,y+1,z] + v_grid[read,x,y,z+1] +
                                v_grid[read,x-1,y,z] + v_grid[read,x,y-1,z] + v_grid[read,x,y,z-1]
                              ) * (1/7);
      }
    }

    var last = 1 - (T&1);

    var failed = false;
    for (x,y,z) in grid_domain {
      failed = grid[last,x,y,z] - v_grid[last,x,y,z] >= epsilon;
      if failed {
        writeln("Failed! " + grid[last,x,y,z] + " - " + v_grid[last,x,y,z] + " >= " + epsilon + " @ (" + x + ", " + y + ", " + z + ")");
        break;
      }
    }

    if !failed {
      writeln("Passed!");
    }
  }

}
