import Foundation
import Glibc

var N = 1000
var T = 100

// Parse arguments
var arg_count = 0;
for argument in Process.arguments{
  switch( argument ){
    case "--N":
      let val = Int( Process.arguments[arg_count+1] )
      if val != nil {
        N = val!
      } else {
        print( "Integer required after --N" )
        exit(-1)
      }

    case "--T":
      let val = Int( Process.arguments[arg_count+1] )
      if val != nil {
        T = val!
      } else {
        print( "Integer required after --T" )
        exit(-1)
      }

    default:
      break // Break out of switch (not loop)
  }
  arg_count += 1
}

var cell_updates = N * N * T
var gflops = Double(cell_updates*5)/1e9

print( "N: \(N)" )
print( "T: \(T)" )

var grid = Array<Array<Array<Double>>>( count: 2, repeatedValue: Array<Array<Double>>(count: N+2, repeatedValue: Array<Double>(count: N+2, repeatedValue: 0.0 ) ) )

let start = NSDate()

for t in 1...T {
  let write = t & 1
  let read = 1 - write

  for i in 1...N {
    for j in 1...N {
      // FIXME:
      // the ususal code
      // grid[write][i][j] = (grid[read][i][j] + grid[read][i+1][j] + grid[read][i][j+1] + grid[read][i-1][j] + grid[read][i][j-1] ) / 5
      // gives the error:
      // error: expression was too complex to be solved in reasonable time; consider breaking up the expression into distinct sub-expressions

      let a = grid[read][i][j] + grid[read][i+1][j]
      let b = grid[read][i][j+1] + grid[read][i-1][j]
      grid[write][i][j] = ( a + b + grid[read][i][j-1] ) / 5
    }
  }

}
let end = NSDate()
let elapsed = end.timeIntervalSinceDate( start )

print( "Elapsed: \(elapsed)s" )
