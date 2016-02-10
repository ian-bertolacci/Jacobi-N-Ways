#!/bin/python
from __future__ import print_function
import yaml, re

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
    print( "something has gone horribly awry." )
    return

  for key in top if isinstance( top, dict ) else xrange(len(top)):
    entry = top[key]

    if isinstance( entry, dict ) or isinstance( entry, list ):
      process_stack( [ entry ] + stack )

    else:
      modified_entry = entry
      for match in variable_rx.finditer( entry ):
        modified_entry = modified_entry.replace( match.group(), find_in_stack( match.group("key"), stack ) )
      top[key] = modified_entry


# Process document's text varialbes
def process_document( document ):
  process_stack( [document] )

if __name__=="__main__":
  stream = file( 'pathways.yaml', 'r' )
  for doc in yaml.load_all(stream):
    process_document(doc)
    print( yaml.dump(doc) )
    print("\n" + (( ("="*10)+"\n")*2) )
