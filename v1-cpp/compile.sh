#!/bin/bash
clear

g++ \
	-x c++ \
	-c upng/upng.c \
	-o /tmp/upng.o

g++ \
	-O3 \
	-Wno-unused-value \
	-Wno-unused-comparison \
	-o shrinkshot \
	/tmp/upng.o \
	shrinkshot.cpp

strip shrinkshot
