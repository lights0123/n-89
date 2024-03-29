DEBUG = FALSE

GCC = nspire-gcc
AS  = nspire-as
GXX = nspire-g++

GCCFLAGS = -Wall -W -marm

ifeq ($(DEBUG),FALSE)
	GCCFLAGS += -O3
else
	GCCFLAGS += -O0 -g
endif

EXCLUDE = ./core/uae/build68k.c ./core/uae/cpustbl.c ./core/uae/fpp.c ./core/uae/gencpu.c ./core/uae/readcpu.c ./core/uae/cpudefs.c ./core/uae/missing.c ./core/uae/cpuemu.c
OBJS = $(patsubst %.c, %.o, $(filter-out $(EXCLUDE), $(shell find . -name \*.c)))
OBJS += $(patsubst %.cpp, %.o, $(shell find . -name \*.cpp))
OBJS += $(patsubst %.S, %.o, $(shell find . -name \*.S))
OBJS += ./core/uae/cpudefs.o ./core/uae/cpustbl.o ./core/uae/readcpu.o ./core/uae/fpp.o ./core/uae/missing.o ./core/uae/cpuemu1.o ./core/uae/cpuemu2.o ./core/uae/cpuemu3.o ./core/uae/cpuemu4.o ./core/uae/cpuemu5.o ./core/uae/cpuemu6.o ./core/uae/cpuemu7.o ./core/uae/cpuemu8.o
LIB = n-89
vpath %.tns $(DISTDIR)
vpath %.elf $(DISTDIR)

all: n-89.tns

uae:
	cd core/uae && $(MAKE) CC="$(GCC)" CFLAGS="$(GCCFLAGS)" gen
	cd core/uae && $(MAKE) CC="$(GCC)" CFLAGS="$(GCCFLAGS)" all

%.o: %.c
	$(GCC) $(GCCFLAGS) -c $< -o $@

%.o: %.cpp
	$(GXX) $(GCCFLAGS) -c $< -o $@
	
%.o: %.S
	$(AS) -c $< -o $@

lib$(LIB).a: uae $(OBJS)
	ar rcs lib$(LIB).a $(OBJS)

n-89.tns: lib$(LIB).a
ifeq (, $(shell which cargo-make))
	cargo install cargo-make
endif
ifeq ($(DEBUG),FALSE)
	cd n-89 && cargo make release
	cp n-89/target/armv5te-nspire-eabi/release/n-89.tns .
else
	cd n-89 && cargo make dev
	cp n-89/target/armv5te-nspire-eabi/debug/n-89.tns .
endif

clean:
	rm -f $(OBJS)
	rm -f n-89.tns libn-89.a
	cd core/uae && $(MAKE) clean && $(MAKE) distclean
