# Directories
BIN=./bin
SRC=./src

#Compilers and flags
CXX=gcc
CFLGS = -fopenmp

EXES = $(BIN)/simple

all: $(EXES)

$(BIN)/simple: $(SRC)/simple.c $(BIN)/util.o
	$(CXX) $^ $(CFLGS) -o $@

$(BIN)/%.o : $(SRC)/%.c
	$(CXX) $^ $(CFLGS) -c -o $@

clean:
	- rm $(BIN)/*
