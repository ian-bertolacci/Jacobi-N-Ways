# Directories
BIN=./bin
SRC=./src

#Compilers and flags
CXX=chpl
CFLGS = --fast

EXES = jacobi_2D_Serial \
			 jacobi_3D_Serial \
			 jacobi_2D_Naive_Parallel \
			 jacobi_3D_Naive_Parallel

LONG_EXES = $(addprefix $(BIN)/,$(EXES))

all: $(LONG_EXES)

$(EXES): % : $(BIN)/%

$(LONG_EXES): $(BIN)/% : $(SRC)/%.chpl
	$(CXX) $^ $(CFLGS) -o $@

clean:
	- rm $(BIN)/*
