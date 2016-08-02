#!/bin/bash
clear

g++ \
	-x c++ \
	-c upng/upng.c \
	-o /tmp/upng.o

g++ \
	-o shrinkshot \
	/tmp/upng.o \
	shrinkshot.cpp

strip shrinkshot
