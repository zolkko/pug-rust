#!/bin/bash

all: sample.so

sample.so: sample.c
	gcc sample.c \
		-shared \
		-Os \
		-o sample.so \
		-I/System/Library/Frameworks/Python.framework/Versions/2.7/include/python2.7 \
		-L/System/Library/Frameworks/Python.framework/Versions/2.7/lib \
		-lpython2.7

dis: sample.so
	otool -tV ./sample.so

.PHONY: clean
clean:
	rm ./sample.so

