use Time;

config var N = 1000;
config var T = 100;

proc main(){

  var iteration_domain: domain(3) = { 1..#N, 1..#N, 1..#N };
  var grid_domain : domain(4) = { 0..1, 0..#N+2, 0..#N+2, 0..#N+2 };

  writeln( "N: " + N );
  writeln( "T: " + T );

  var grid : [grid_domain] real(64);

  var timer : Timer;

  timer.start();

  for t in 1..T {
    var read = t & 1;
    var write = 1 - read;
    forall (x,y,z) in iteration_domain{
      grid[write,x,y,z] = ( grid[read,x,y,z] +
                            grid[read,x+1,y,z] + grid[read,x,y+1,z] + grid[read,x,y,z+1] +
                            grid[read,x-1,y,z] + grid[read,x,y-1,z] + grid[read,x,y,z-1]
                          ) / 5;
    }
  }

  timer.stop();

  writeln( "Elapsed: " + timer.elapsed() + "s" );

}
