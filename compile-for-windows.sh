#!/bin/bash
clear

i686-w64-mingw32-g++ \
	-x c++ \
	-c upng/upng.c \
	-o /tmp/upng-w.o

i686-w64-mingw32-g++ \
	-O3 \
	-Wno-unused-value \
	-Wno-unused-comparison \
	-o shrinkshot.exe \
	/tmp/upng-w.o \
	shrinkshot.cpp \
	-static \


strip shrinkshot
