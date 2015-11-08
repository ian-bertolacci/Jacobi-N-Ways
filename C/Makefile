# Directories
BIN=./bin
SRC=./src

#Compilers and flags
CXX=gcc
CFLGS = -fopenmp

EXES = $(BIN)/simple

all: $(EXES)

$(BIN)/simple: $(SRC)/Jacobi.c
	$(CXX) $^ $(CFLGS) -o $@

clean:
	- rm $(BIN)/*
