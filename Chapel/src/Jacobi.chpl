use Time;

config var N = 1000;
config var T = 100;

proc main(){

  var iteration_domain: domain(2) = { 1..#N, 1..#N };
  var grid_domain : domain(3) = { 0..1, 0..#N+2, 0..#N+2 };

  var grid : [grid_domain] real(64);

  var timer : Timer;

  timer.start();

  for t in 1..T {
    var read = t & 1;
    var write = 1 - read;
    for (i,j) in iteration_domain{
      grid[write,i,j] = ( grid[read,i,j] +
                          grid[read,i+1,j] + grid[read,i,j+1] +
                          grid[read,i-1,j] + grid[read,i,j-1] ) / 5;
    }
  }

  timer.stop();

  writeln( "Cell Updates: " + T*iteration_domain.size );
  writeln( "GFLOPS: " + (T*iteration_domain.size*5)/1e9 );
  writeln( "Elapsed: " + timer.elapsed() + "s" );
  writeln( "Per Cell Update: " + (timer.elapsed()/( T*iteration_domain.size )) + "s" );
  writeln( "GFLOPS/s: " + (( T*iteration_domain.size*5 )/timer.elapsed())/1e9  );

}
