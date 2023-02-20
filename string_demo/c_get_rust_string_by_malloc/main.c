#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>

typedef void *(*Allocator)(uintptr_t);

char *get_string_with_allocator(Allocator allocator);

/**
 *这里展示与缓冲区方式不一样的函数copy api
 */
void copy_string(char *ptr);

void free_string(const char *ptr);

int main() {
  char* rust_string = get_string_with_allocator(malloc);
  printf("%s\n",rust_string);
  free(rust_string);  //This use free not free_string
}