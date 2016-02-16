# Directories
BIN=./bin
CLASS=./class
SRC=./src
LIB=./lib

# Libs
APACHE_LANG_3_LIB=$(LIB)/commons-lang3-3.4/commons-lang3-3.4.jar
APACHE_CLI_LIB=$(LIB)/commons-cli-1.3.1/commons-cli-1.3.1.jar
LIBS = $(APACHE_LANG_3_LIB) $(APACHE_CLI_LIB)

#Compilers and flags
CXX=javac
JAR=jar
CFLGS =


EXES = simple.jar
LONG_EXES = $(addprefix $(BIN)/,$(EXES))

all: $(LONG_EXES)

$(EXES): % : $(BIN)/%

$(BIN)/simple.jar: $(LIBS) $(SRC)/Simple_Main.java
	$(CXX) -cp $(subst $(empty) $(empty),:,$(LIBS) $(SRC)) -d $(CLASS) $(SRC)/Simple_Main.java
	echo "$(LIBS)"
	cd $(CLASS) ; \
	echo "Main-Class: Simple_Main" > manifest.txt ; \
	$(foreach lib,$(LIBS),$(JAR) xf .$(lib) ; )
	cd $(CLASS) ; \
	$(JAR) cvfm ../$@ manifest.txt *

$(APACHE_LANG_3_LIB):
	cd $(LIB); \
	unzip commons-lang3-3.4-bin.zip

$(APACHE_CLI_LIB):
	cd $(LIB); \
	unzip commons-cli-1.3.1-bin.zip

neat:
	- rm -r $(CLASS)/*

clean: neat
	- rm $(BIN)/*
	- rm -r $(LIB)/commons-lang3-3.4 $(LIB)/commons-cli-1.3.1
