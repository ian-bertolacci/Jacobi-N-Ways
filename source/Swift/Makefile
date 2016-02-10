# Directories
BIN=./bin
SRC=./src

#Compilers and flags
CXX=swiftc
CFLGS = -O

EXES = simple
LONG_EXES = $(addprefix $(BIN)/,$(EXES))

all: $(LONG_EXES)

$(EXES): % : $(BIN)/%

$(LONG_EXES): $(BIN)/% : $(SRC)/%.swift
	$(CXX) $^ $(CFLGS) -o $@

clean:
	- rm $(BIN)/*
