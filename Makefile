# Directories
BIN=./bin
SRC=./src

#Compilers and flags
C_CPLR=gcc

CPP_CPLR=g++

CHPL_CPLR=chpl

JAVA_CPLR=javac

EXES = $(BIN)/Jacobi_C_simple

$(BIN)/Jacobi_C_simple: $(SRC)/C/Jacobi.c
	$(C_CPLR) $^  -fopenmp -o $@

clean:
	- rm $(BIN)/*
