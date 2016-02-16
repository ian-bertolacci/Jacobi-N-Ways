import os, sys

'''
class DirectoryStack:
  Purpose:
    Emulate pushd and popd bash commands

  Member Functions:
    __init__( stdout = sys.stdout ):
      Purpose:
        Constructor.

      Parameters:
        stdout:
          (optional, defaults to sys.stdout) file/stream to print stack state
          to.

    pushd( path ):
      Purpose:
        push pwd onto self.stack, chdir to directory specified in path.

      Parameters:
        path:
          full or relative path to directory.

    popd( ):
      Purpose:
        pop a directory from self.stack, chdir to that location.
        If stack is empty, prints error message and does not chdir.

    print_stacK( ):
      Purpose:
        print the current state of the stack to self.stdout

  Member Variables:
    stack:
      list of directories, behaves as stack.

    stdout:
      file/stream to print stack state to.
'''
class DirectoryStack:
  def __init__( self, stdout = sys.stdout ):
    self.stack = []
    self.stdout = stdout

  def pushd( self, path ):
    # Store pwd, then cd to path
    self.stack.append( os.getcwd() )
    os.chdir( path )
    self.print_stack()


  def popd( self ):
    if len( self.stack ) > 0:
      # cd to first path on stack
      os.chdir( self.stack.pop() )
      self.print_stack()
    else:
      self.stdout.write( "popd: Directory stack empty\n" );

  def print_stack( self ):
    self.stdout.write( " ".join( self.stack + [os.getcwd()] ) + "\n" )
