#!/bin/bash

# clean coverage files
rm -f *.gcda *.c.gcov

../target/debug/short

# Coverage
gcov main.rs
