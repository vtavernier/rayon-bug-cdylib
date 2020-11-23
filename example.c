#include <stdlib.h>
#include <stdio.h>
#include <stddef.h>
#include <stdbool.h>

struct ThreadPool;
typedef struct ThreadPool ThreadPool;

extern ThreadPool* new_rayon_pool();                  // from crate_a
extern void do_something_with_pool(ThreadPool *pool); // from_crate_b

int main(int argc, char *argv[]) {
	ThreadPool *pool = new_rayon_pool(); // works
	do_something_with_pool(pool);        // assertion fails
	return 0;
}
