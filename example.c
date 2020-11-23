#include <stdlib.h>
#include <stdio.h>
#include <stddef.h>
#include <stdbool.h>

struct ThreadPool;
typedef struct ThreadPool ThreadPool;

extern ThreadPool* new_rayon_pool();
extern void do_something_with_pool(ThreadPool *pool);

int main(int argc, char *argv[]) {
	ThreadPool *pool = new_rayon_pool();
	do_something_with_pool(pool);
	return 0;
}
