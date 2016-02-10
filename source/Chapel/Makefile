# Directories
BIN=./bin
SRC=./src

#Compilers and flags
CXX=chpl
CFLGS = --fast

EXES = simple
LONG_EXES = $(addprefix $(BIN)/,$(EXES))

all: $(LONG_EXES)

$(EXES): % : $(BIN)/%

$(LONG_EXES): $(BIN)/% : $(SRC)/%.chpl
	$(CXX) $^ $(CFLGS) -o $@

clean:
	- rm $(BIN)/*
