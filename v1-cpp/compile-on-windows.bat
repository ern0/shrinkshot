#!/bin/bash
cls

g++ ^
	-x c++ ^
	-c upng\upng.c ^
	-o %TEMP%\upng.o

g++ ^
	-O3 ^
	-Wno-unused-value ^
	-Wno-unused-comparison ^
	-o shrinkshot.exe ^
	%TEMP%\upng.o ^
	-D WINDOWS ^
	shrinkshot.cpp ^
	-static
