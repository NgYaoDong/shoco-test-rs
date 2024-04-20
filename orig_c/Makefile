# compile shoco_lib_test.c 
all: shoco_lib_test_dbg shoco_lib_test_opt shoco_lib_test

shoco_lib_test: shoco_lib_test.c
	gcc -O0 -fprofile-arcs -ftest-coverage -o shoco_lib_test shoco_lib_test.c

shoco_lib_test_dbg: shoco_lib_test.c
	gcc -g -O0 -o shoco_lib_test_dbg shoco_lib_test.c

shoco_lib_test_opt: shoco_lib_test.c
	gcc -O3 -o shoco_lib_test_opt shoco_lib_test.c

clean:
	rm -f shoco_lib_test_dbg shoco_lib_test shoco_lib_test_opt *.gcda *.gcno *.gcov