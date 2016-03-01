#!/bin/python
from __future__ import print_function
from DirectoryStack import DirectoryStack
import yaml, re, argparse, subprocess, sys, os, argparse

sys.stdnull = open( "/dev/null", "w")

variable_rx = re.compile( r"\${2}(?P<key>\w+)" )

# Return a particular value from dictionaries in the stak using the given key
def find_in_stack( key, stack ):
  top = stack[0]
  if isinstance( top, list ) or key not in top:
    return find_in_stack( key, stack[1:] )
  else:
    return top[key]

# Recursive entry to process_build.
# Recursively iterate through stack and text replace all variables.
def process_stack( stack ):
  top = stack[0]

  if( not ( isinstance(top, dict) or isinstance(top, list) ) ):
    print( "Item on top of processing stack is not a dictionary or list." )
    return

  for key in top if isinstance( top, dict ) else xrange(len(top)):
    entry = top[key]

    # First stage processing.
    # Recurse on stack
    if isinstance( entry, dict ) or isinstance( entry, list ):
      process_stack( [ entry ] + stack )

    # Variable replacement
    elif isinstance( entry, str ):
      modified_entry = entry
      for match in variable_rx.finditer( entry ):
        modified_entry = modified_entry.replace( match.group(), find_in_stack( match.group("key"), stack ) )
      top[key] = modified_entry

    # Reset entry after any potential modification
    entry = top[key]

    # Second stage processing
    # command keys need to be converted to lists
    if str(key) == "command" and isinstance(entry, str):
      top[key] = re.split(r"\s+", entry)


# Process document's text varialbes
def process_document( document ):
  process_stack( [document] )

class FailedBuildException( Exception ):
  def __init__(self, language, variant, message=""):
    super( Exception, self).__init__( "{0}:{1} failed to build{2}".format(language, variant, "!" if message == "" else ":\n{0}".format(message) ) )


class FailedRunException( Exception ):
  def __init__(self, language, variant):
    super( Exception, self).__init__( "{0}:{1} failed to run{2}".format(language, variant, "!" if message == "" else ":\n{0}".format(message) ) )

class FailedCleanException( Exception ):
  def __init__(self, language, variant):
    super( Exception, self).__init__( "{0}:{1} failed to clean{2}".format(language, variant, "!" if message == "" else ":\n{0}".format(message) ) )

class ReadableOutput:
  def __init__(self):
    self.string = ""

  def write( self, string ):
    self.string += string

  def __str__(self):
    return self.string


class LanguageRunner:

  def __init__(self, language_doc, log_file, N_range, T_range, variants = [], stdout = sys.stdout, stderr = sys.stderr ):
    self.language_doc = language_doc;
    self.N_range = N_range
    self.T_range = T_range
    self.running_variants = variants;

    self.log = log_file
    self.stdout = stdout
    self.stderr = stderr

    self.dir_stack = DirectoryStack( self.log )

  def run(self):
    self.log.write( "[Running variants for {0}]\n".format(self.language_doc["name"]) )

    try:
      for variant in self.language_doc["variations"]:
        if self.running_variants == [] or variant["name"] in self.running_variants:
          self.run_variant( variant )

    except:
      self.log.flush()
      raise


  def run_variant(self, variant_doc ):

    self.log.write( "[Running variant {0}]\n".format(variant_doc["name"]) )

    build_out = ReadableOutput()
    build_err = ReadableOutput()

    exit_code = self.operation( variant_doc, "build", fail_if_missing = False, stdout = build_out, stderr = build_err )

    if exit_code != 0:
      raise FailedBuildException( self.language_doc["name"], variant_doc["name"], message = "{0}\n{1}".format( build_out, build_err ) )

    for iterations in self.T_range:
      if "max" in variant_doc["options"]["iterations"] and iterations > variant_doc["options"]["iterations"]["max"]:
        self.log.write("[Iterations over maximum variant value: {0} > {1}]\n".format(iterations, variant_doc["options"]["iterations"]["max"]) )
        break

      for size in self.N_range:
        if "max" in variant_doc["options"]["size"] and size > variant_doc["options"]["size"]["max"]:
          self.log.write("[Size over maximum variant value: {0} > {1}]\n".format(size, variant_doc["options"]["size"]["max"]) )
          break

        arg_builder = lambda arg,value: [variant_doc["options"][arg]["arg"], str(value)]
        options = reduce( list.__add__, map( arg_builder, ["size","iterations"], [size,iterations] ) )

        self.stdout.write( self.execution_preamble( variant_doc, size, iterations ) + "\n" )

        run_out = ReadableOutput()
        run_err = ReadableOutput()

        exit_code = self.operation( variant_doc, "run", options = options, stdout = run_out, stderr = run_err )

        if exit_code != 0:
          raise FailedRunException( self.language_doc["name"], variant_doc["name"], message = "{0}\n{1}".format( run_out, run_err ) )

        elapsed_rx = re.compile( r"Elapsed:\s+(?P<elapsed>[-+]?[0-9]+[.]?[0-9]*(?:[eE][-+]?[0-9]+)?)s" )

        elapsed = float( elapsed_rx.search( str(run_out) ).group("elapsed") )

        self.stdout.write( self.execution_postscript( variant_doc, size, iterations, elapsed ) + "\n" + ("="*10) + "\n")


    clean_out = ReadableOutput()
    clean_err = ReadableOutput()

    exit_code = self.operation( variant_doc, "clean", fail_if_missing = False, stdout = clean_out, stderr = clean_err )

    if exit_code != 0:
      raise FailedCleanException( self.language_doc["name"], variant_doc["name"], message = "{0}\n{1}".format( clean_out, clean_err ) )

  def operation(self, variant_doc, operation, options = [], fail_if_missing = True, stdout = None, stderr = None):
    stdout = self.stdout if stdout == None else stdout
    stderr = self.stderr if stderr == None else stderr

    self.log.write( "[Performing '{0}' operation ]\n".format(operation) )

    if operation in variant_doc:
      self.dir_stack.pushd(  "./{0}/".format( variant_doc[operation]["directory"] )  )

      command_list = variant_doc[operation]["command"] + options
      self.log.write( str(command_list) )
      self.log.write( "{0}\n".format( " ".join(command_list) ) )

      process = subprocess.Popen( command_list, stdout=subprocess.PIPE, stderr=subprocess.PIPE  )

      p_stdout, p_stderr = process.communicate()
      exit_code = process.returncode

      stdout.write( p_stdout )
      self.log.write( "StdOut:\n{0}\n{1}\n{0}\n".format( "="*10, p_stdout ) )

      stderr.write( p_stderr )
      self.log.write( "StdError:\n{0}\n{1}\n{0}\n".format( "="*10, p_stderr ) )

      self.log.write( "[Exit status: {0}]\n".format(exit_code) )

      self.dir_stack.popd()

      return exit_code

    elif not fail_if_missing :
      self.log.write( "[No such operation '{0}', legally skipping]\n".format(operation) )
      return 0

    self.log.write( "[ERROR: No such operation '{0}', FAILING]\n".format(operation) )
    raise Exception( "{0} is not declared in the run doc for {1}:{2}".format( operation, language_doc["name"], variant_doc["name"]) )

  def execution_preamble( self, variant_doc, N, T ):
    return "\n".join(
                      [ "Language: {0}".format( self.language_doc["name"] ),
                        "Variant: {0}".format( variant_doc["name"] ),
                        "Size: {0}".format( str(N) ),
                        "Iterations: {0}".format( str(T) ),
                        "Cell Updates: {0}".format( str(N*T) ),
                        "GFLOPS: {0}".format( str( (N*T*variant_doc["flops"])/10e9 ) )
                      ]
                    )

  def execution_postscript( self, variant_doc, N, T, elapsed ):

    GFLOPSS = ((N*T*variant_doc["flops"])/10e9 )/elapsed if elapsed != 0.0 else float("inf")

    return "\n".join(
                      [ "Elapsed: {0}".format( elapsed ),
                        "GFLOPS/s: {0}".format( str(GFLOPSS ) )
                      ]
                    )


def main():

  parser = argparse.ArgumentParser()
  parser.add_argument( "--langs",            dest="langs",            type=str, nargs="+", metavar="Language" )
  parser.add_argument( "--variants",         dest="vars",             type=str, nargs="+", metavar="Variant" )
  parser.add_argument( "--size",             dest="size",             type=int, nargs=1,   metavar="Size" )
  parser.add_argument( "--size_range",       dest="size_range",       type=int, nargs=3,   metavar=("start","maximum","increment") )
  parser.add_argument( "--iterations",       dest="iterations",       type=int, nargs=1,   metavar="Iterations" )
  parser.add_argument( "--iterations_range", dest="iterations_range", type=int, nargs=3,   metavar=("start","maximum","increment") )

  args = parser.parse_args()

  if args.size != None and args.size_range != None:
    print("Incompatable flags --size and --size_range were used")
    exit()

  elif args.size != None:
    size_range = xrange( args.size[0], args.size[0]+1, 1)

  elif args.size_range != None:
    size_range = xrange( args.size_range[0], args.size_range[1]+1, args.size_range[2] )
  else:
    size_range = xrange(100,1000,100)

  if args.iterations != None and args.iterations_range != None:
    print("Incompatable flags --iterations and --iterations_range were used")
    exit()

  elif args.iterations != None:
    iterations_range = xrange( args.iterations[0], args.iterations[0]+1, 1)

  elif args.iterations_range != None:
    iterations_range = xrange( args.iterations_range[0], args.iterations_range[1]+1, args.iterations_range[2] )

  else:
    iterations_range = [1] + range(100,1000,50)

  variants = [] if args.vars == None else args.vars
  languages = set(["All"]) if args.langs == None else set( args.langs )


  try:
    os.remove( "./logfile.log" )
  except OSError:
    pass # Leave this pass. No action of failure.

  log = open( "./logfile.log", "w")
  stream = file( "pathways.yaml", "r" )
  first_bar = True
  for doc in yaml.load_all(stream):
    process_document(doc)
    if   ( ("All" in languages)  and ( "run" not in doc or doc["run"] ) ) \
      or ( doc["name"] in languages ) :
      if first_bar:
        sys.stdout.write( ("="*10) + "\n" )
        first_bar = False
      test = LanguageRunner(doc, log, variants = variants, N_range = size_range, T_range = iterations_range)
      test.run()


if __name__=="__main__":
  main()
