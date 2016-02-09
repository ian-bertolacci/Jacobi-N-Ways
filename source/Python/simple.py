import numpy as np
import time, argparse



if __name__ == "__main__":

  parser = argparse.ArgumentParser(description='Python version of Jacobi 2D')
  parser.add_argument( '--N', metavar="N", dest='N', type=int, action='store', default=1000)
  parser.add_argument( '--T', metavar="T", dest='T', type=int, action='store', default=100)

  args = parser.parse_args()

  N = args.N
  T = args.T

  updates = ((N**2)*T)
  gflops = (updates*5)/1e9

  print "N: {0}".format( N )
  print "T: {0}".format( T )
  print "Cell Updates: {0}".format( updates )
  print "GFLOPS: {0}".format( gflops )

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

  print "Elapsed: {0}s".format( elapsed )
  print "Per Cell Update: {0}s".format( elapsed/updates )
  print "GFLOPS/s: {0}".format( gflops/elapsed )
