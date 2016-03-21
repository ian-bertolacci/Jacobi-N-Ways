use Time;

config var N = 1000;
config var T = 100;

config var tile_size = N / ( if dataParTasksPerLocale != 0 then dataParTasksPerLocale else here.numCores );
var tiles = max( ceil( (1.0*N) / tile_size) : int, 1); // If tile_size unset === cores/dataParTasks
//writeln( (tile_size, tiles*tiles)  );

proc main(){

  var iteration_domain: domain(2) = { 1..#N, 1..#N };
  var tile_subdomain : domain(2) = { 1..#tile_size, 1..#tile_size };
  var grid_domain : domain(3) = { 0..1, 0..#N+2, 0..#N+2 };
 /*
  writeln( "N: " + N );
  writeln( "T: " + T );
  */
  var grid : [grid_domain] real(64);

  var timer : Timer;

  timer.start();

  for t in 1..T {
    const read = t & 1;
    const write = 1 - read;
    forall tile_ in 0..#tiles*tiles with (ref grid ){
      const tile_x = tile_ / tiles;
      const tile_y = tile_ % tiles;
      const start_x = (tile_size * tile_x)+1;
      const start_y = (tile_size * tile_y)+1;
      const end_x = min( start_x + tile_size-1, N );
      const end_y = min( start_y + tile_size-1, N );
      const tile_domain : domain(2) = { start_x .. end_x, start_y .. end_y } ;
      //writeln( (tile_, tile_x, tile_y, (start_x, end_x), (start_y, end_y), tile_domain) );
      //*
      for (x,y) in tile_domain { //tile_subdomain.translate(tile_size * tile_x, tile_size * tile_y){
        grid[write,x,y] = ( grid[read,x,y] +
                            grid[read,x+1,y] + grid[read,x,y+1] +
                            grid[read,x-1,y] + grid[read,x,y-1] ) / 5;
      } // */
      //}
    }
  }

  timer.stop();

  writeln( "Elapsed: " + timer.elapsed() + "s" );

}
