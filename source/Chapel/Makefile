# Directories
BIN=./bin
SRC=./src

#Compilers and flags
CXX=chpl
CFLGS = --fast

EXES = $(BIN)/simple

all: $(EXES)

$(BIN)/simple: $(BIN)/% : $(SRC)/%.chpl
	$(CXX) $^ $(CFLGS) -o $@

clean:
	- rm $(BIN)/*
