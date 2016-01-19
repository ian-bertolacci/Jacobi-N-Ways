#include "util.h"

struct ProgramOptions parseArguments( char** argv, int argc ){
  struct ProgramOptions opts;
  opts.N = 1000;
  opts.T = 100;

  int c;
  opterr = 0;

  while( (c = getopt (argc, argv, "N:T:")) != -1){
    switch (c){
      case 'N':
        opts.N = atoi( optarg );
        break;

      case 'T':
        opts.T = atoi( optarg );
        break;

      case '?':
        printf( "?\n" );
        if( optopt == 'N' || optopt == 'T' )
          fprintf (stderr, "Option -%c requires an argument.\n", optopt);

        else if (isprint (optopt))
          fprintf (stderr, "Unknown option `-%c'.\n", optopt);

        else
          fprintf (stderr,
                   "Unknown option character `\\x%x'.\n",
                   optopt);

        abort();
      default:
        printf( "default\n" );
        abort ();
    }
  }

  return opts;
}
