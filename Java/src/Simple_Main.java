import java.util.Timer;
import org.apache.commons.lang3.time.StopWatch;

public class Simple_Main {

  public static void main( String[] args ){
    int grid_size = 1000;
    int iterations = 100;

    double[][][] grid = new double[2][grid_size+2][grid_size+2];

    StopWatch timer = new StopWatch();

    timer.start();

    for( int t = 0, read = 0, write = 1; t < iterations; t += 1, read = 1-read, write = 1-write ){
      for( int i = 1; i <= grid_size; i += 1 ){
        for( int j = 1; j <= grid_size; j += 1 ){
          grid[write][i][j] = ( grid[read][i][j] +
                                grid[read][i+1][j] + grid[read][i][j+1] +
                                grid[read][i-1][j] + grid[read][i][j-1] ) / 5;
        }
      }
    }

    timer.stop();
    int iteration_size = (grid_size-2)*(grid_size-2);
    int cell_updates = iteration_size * iterations;
    double gflops = (cell_updates*5)/1e9;
    double seconds = timer.getTime() / 1000.0;

    System.out.println( "Cell Updates: " + cell_updates );
    System.out.println( "GFLOPS: " + gflops );
    System.out.println( "Elapsed: " + seconds + "s" );
    System.out.println( "Per Cell Updates: " + (seconds/cell_updates) );
    System.out.println( "GFLOPS/s: " + (gflops/seconds) );

  }

}
