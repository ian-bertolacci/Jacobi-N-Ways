import numpy as np
import time

if __name__ == "__main__":
  N = 1000
  T = 100

  grid = [ [[0]*(N+2)]*(N+2) ]*2
  start = time.time()

  for t in xrange(1,T+1):
    write = t & 1
    read = 1 - write
    for i in xrange(1,N+1):
      for j in xrange(1,N+1):
        grid[write][i][j] = ( grid[read][i][j] +
                              grid[read][i+1][j] + grid[read][i][j+1] +
                              grid[read][i-1][j] + grid[read][i][j-1] ) / 5

  end = time.time()
  elapsed = end - start
  print "Elapsed: " + str(elapsed) + "s"
