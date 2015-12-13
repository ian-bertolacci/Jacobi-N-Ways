# Directories
BIN=./bin
CLASS=./class
SRC=./src
LIB=./lib

# Libs
APACHE_LIB = $(LIB)/commons-lang3-3.4/commons-lang3-3.4.jar

#Compilers and flags
CXX=javac
JAR=jar
CFLGS =
JARFLGS=cvfm

EXES = $(BIN)/simple.jar

all: $(EXES)

$(BIN)/simple.jar: $(APACHE_LIB) $(SRC)/Simple_Main.java
	$(CXX) -cp $(APACHE_LIB):$(SRC) -d $(CLASS) $(SRC)/Simple_Main.java
	cd $(CLASS) ; \
	echo "Main-Class: Simple_Main" > manifest.txt ; \
	$(JAR) xf ../$(APACHE_LIB) org/ ; \
	$(JAR) $(JARFLGS) ../$@ manifest.txt * ; \
	cd ..
	$(MAKE) neat

$(APACHE_LIB):
	cd $(LIB); \
	unzip commons-lang3-3.4-bin.zip

neat:
	- rm -r $(CLASS)/*

clean: neat
	- rm $(BIN)/*
	- rm -r $(LIB)/commons-lang3-3.4
