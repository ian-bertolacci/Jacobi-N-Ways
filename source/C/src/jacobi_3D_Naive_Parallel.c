#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <time.h>
#include <omp.h>
#include "util.h"

typedef struct _grid_struct{
  double*** data;
  int size;
} Grid;

Grid* grid_alloc( int grid_size ){
  Grid* result = (Grid*) malloc( sizeof(Grid) );

  result->data = malloc( (grid_size+2) * sizeof( double** ) );
  for( int i = 0; i <= grid_size+1; i += 1 ){
    result->data[i] = calloc( grid_size+2, sizeof( double* ) );
    for( int j = 0; j <= grid_size+1; j += 1 ){
      result->data[i][j] = calloc( grid_size+2, sizeof( double ) );
    }
  }

  result->size = grid_size;
  return result;
}

void grid_dealloc( Grid* grid ){
  for( int i = 0; i <= grid->size+1; i += 1 ){
    for( int j = 0; j <= grid->size+1; j += 1 ){
        free( grid->data[i][j] );
    }
    free( grid->data[i] );
  }
  free( grid->data );
  free( grid );
}

void populate_grid( Grid* grid ){
  srand (time(NULL));
  for( int i = 1; i <= grid->size; i += 1 ){
    for( int j = 1; j <= grid->size; j += 1 ){
      for( int k = 1; k <= grid->size; k += 1 ){
        grid->data[i][j][k] = rand();
      }
    }
  }
}

static inline void step( Grid* source, Grid* target ){
  #pragma omp parallel for
  for( int i = 1; i <= source->size; i += 1 ){
    for( int j = 1; j <= source->size; j += 1 ){
      for( int k = 1; k <= source->size; k += 1 ){
        target->data[i][j][k] = ( source->data[i][j][k] +
                                  source->data[i+1][j][k] + source->data[i][j+1][k] + source->data[i][j][k+1] +
                                  source->data[i-1][j][k] + source->data[i][j-1][k] + source->data[i][j][k-1] ) / 7.0;
      }
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
  Grid* grid_r = grid_alloc( grid_size );
  Grid* grid_w = grid_alloc( grid_size );
  Grid* result = NULL;

  populate_grid( grid_r );

  double start = omp_get_wtime();

  // Do steps in pairs
  for( int t = 0; t < iterations; t += 1 ){
    step( grid_r, grid_w );
    result = grid_w;
    grid_w = grid_r;
    grid_r = result;
  }


  double end = omp_get_wtime();
  double elapsed = end - start;

  printf( "Elapsed: %fs\n", elapsed );

  grid_dealloc( grid_r );
  grid_dealloc( grid_w );

  return 0;
}
