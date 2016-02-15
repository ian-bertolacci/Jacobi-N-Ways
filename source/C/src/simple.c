#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <time.h>
#include <omp.h>
#include "util.h"

typedef struct _grid_struct{
  double** data;
  int size;
} Grid;

Grid* grid_alloc( int grid_size ){
  Grid* result = (Grid*) malloc( sizeof(Grid) );

  result->data = malloc( (grid_size+2) * sizeof( double* ) );
  for( int i = 0; i <= grid_size+1; i += 1 ){
    result->data[i] = calloc( grid_size+2, sizeof( double ) );
  }

  result->size = grid_size;
  return result;
}

void grid_dealloc( Grid* grid ){
  for( int i = 0; i <= grid->size+1; i += 1 ){
    free( grid->data[i] );
  }
  free( grid->data );
  free( grid );
}

void populate_grid( Grid* grid ){
  srand (time(NULL));
  for( int i = 1; i <= grid->size; i += 1 ){
    for( int j = 1; j <= grid->size; j += 1 ){
      grid->data[i][j] = rand();
    }
  }
}

static inline void step( Grid* source, Grid* target ){
  for( int i = 1; i <= source->size; i += 1 ){
    for( int j = 1; j <= source->size; j += 1 ){
      target->data[i][j] = ( source->data[i][j] +
                             source->data[i+1][j] + source->data[i][j+1] +
                             source->data[i-1][j] + source->data[i][j-1] ) / 5;
    }
  }
}

int main( int argc, char** argv ){

  struct ProgramOptions opts = parseArguments( argv, argc );

  int grid_size = opts.N;
  int iterations = opts.T;

  printf( "N: %d\n", grid_size );
  printf( "T: %d\n", iterations );

  // allocate ping-pong grids
  Grid* grid_a = grid_alloc( grid_size );
  Grid* grid_b = grid_alloc( grid_size );
  Grid* result = NULL;

  populate_grid( grid_a );

  double start = omp_get_wtime();

  // Do steps in pairs
  for( int t = 0; t < iterations; t += 2 ){
    step( grid_a, grid_b );
    step( grid_b, grid_a );
  }

  // Set result
  // if odd number of iterations, do remainder
  if( iterations & 1 == 1 ){
    step( grid_a, grid_b );
    result = grid_b;
  } else {
    result = grid_a;
  }

  double end = omp_get_wtime();
  double elapsed = end - start;

  printf( "Elapsed: %fs\n", elapsed );

  grid_dealloc( grid_a );
  grid_dealloc( grid_b );

  return 0;
}
