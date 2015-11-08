# Directories
BIN=./bin
SRC=./src

#Compilers and flags
CXX=gcc
CFLGS = -fopenmp

EXES = $(BIN)/Jacobi_C_simple

all: $(EXES)

$(BIN)/Jacobi_C_simple: $(SRC)/Jacobi.c
	$(CXX) $^  $(CFLGS) -o $@

clean:
	- rm $(BIN)/*
