#ifndef C_JACOBI_N_WAYS_UTIL_H_9BjtT
#define C_JACOBI_N_WAYS_UTIL_H_9BjtT

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <ctype.h>

struct ProgramOptions {
  int T;
  int N;
};

struct ProgramOptions parseArguments( char** argv, int argc );

#endif
