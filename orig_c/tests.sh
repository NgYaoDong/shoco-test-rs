#!/bin/bash

# clean coverage files
rm -f *.gcda *.c.gcov

./shoco_lib_test

# Coverage
gcov shoco_lib_test.c
