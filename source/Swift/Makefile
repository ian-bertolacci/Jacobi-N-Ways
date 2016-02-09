# Directories
BIN=./bin
SRC=./src

#Compilers and flags
CXX=swiftc
CFLGS = -O

EXES = $(BIN)/simple

all: $(EXES)

$(BIN)/simple: $(BIN)/% : $(SRC)/%.swift
	$(CXX) $^ $(CFLGS) -o $@

clean:
	- rm $(BIN)/*
