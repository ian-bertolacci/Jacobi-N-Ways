import org.apache.commons.cli.*;
import java.lang.RuntimeException;
import java.lang.Integer;

class ProjectCommandLineParser {
  public CommandLineParser parser;
  public Options options;
  public CommandLine parsed;

  ProjectCommandLineParser( ){
    this.parser = new DefaultParser();
    this.options = new Options();
    this.options.addOption( "N", true, "Edge length of computed grid." );
    this.options.addOption( "T", true, "Iterations performed" );
  }

  public CommandLine parse( String[] args ) throws ParseException{
    this.parsed = this.parser.parse( this.options, args );
    return this.parsed;
  }

  public String get( String arg ){
    if( this.parsed == null ){
      throw new RuntimeException( "Must parse arguments first" );
    }
    return this.parsed.getOptionValue( arg );
  }

  public Integer getN( ){
    String value = this.get("N");
    return (value == null )? 1000 : new Integer( value );
  }

  public Integer getT( ){
    String value = this.get("T");
    return (value == null )? 100 : new Integer( value );
  }

}
