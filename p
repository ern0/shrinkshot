#!/bin/bash
clear

g++ \
	-x c++ \
	-c upng/upng.c \
	-o upng.o

g++ \
	-o shrinkshot \
	upng.o \
	shrinkshot.cpp

strip shrinkshot
