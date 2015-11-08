# Directories
BIN=./bin
SRC=./src

#Compilers and flags
CXX=chpl
CFLGS = --fast

EXES = $(BIN)/simple

all: $(EXES)

$(BIN)/simple: $(SRC)/simple.chpl
	$(CXX) $^ $(CFLGS) -o $@

clean:
	- rm $(BIN)/*
