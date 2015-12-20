import numpy as np
import time, random, copy

class xcart:
  def __init__( self, lx, ux, ly, uy ):
    self.lx = lx
    self.ly = ly
    self.ux = ux
    self.uy = uy
    self.i = lx-1
    self.j = ly

  def __iter__(self):
    return self

  def next( self ):
    self.i += 1
    if self.i <= self.ux:
      return (self.i,self.j)
    else:
      self.j += 1
      if self.j <= self.uy:
        self.i = self.lx
        if self.i <= self.ux:
          return (self.i,self.j)
      else:
        raise StopIteration()

if __name__ == "__main__":
  N = 1000
  T = 100

  grid = [ [ [0]*(N+2)]*(N+2) ]*2

  start = time.time()

  def timestep( t, read, write ):
    def jacobi( i,j ):
      write[i][j] = ( read[i][j] +
                      read[i+1][j] + read[i][j+1] +
                      read[i-1][j] + read[i][j-1] ) / 5

    for (i,j) in xcart(1,N, 1,N):
      jacobi(i,j)

    # tail recusion
    if t > 1:
      timestep( t-1, grid[1], grid[0] )

  timestep( T, grid[0], grid[1] )

  end = time.time()
  elapsed = end - start
  updates = ((N**2)*T)
  gflops = (updates*5)/1e9

  print "Cell Updates: {0}".format( updates )
  print "GFLOPS: {0}".format( gflops )
  print "Elapsed: {0}s".format( elapsed )
  print "Per Cell Update: {0}s".format( elapsed/updates )
  print "GFLOPS/s: {0}".format( gflops/elapsed )
