# Directories
BIN=./bin
SRC=./src

#Compilers and flags
CXX=gcc
CFLGS = -fopenmp

EXES = simple
LONG_EXES = $(addprefix $(BIN)/,$(EXES))

all: $(LONG_EXES)

$(EXES): % : $(BIN)/%

$(LONG_EXES): $(BIN)/% : $(SRC)/%.c $(BIN)/util.o
	$(CXX) $^ $(CFLGS) -o $@

#$(BIN)/simple: $(SRC)/simple.c $(BIN)/util.o
#	$(CXX) $^ $(CFLGS) -o $@

$(BIN)/%.o : $(SRC)/%.c
	$(CXX) $^ $(CFLGS) -c -o $@

clean:
	- rm $(BIN)/*
