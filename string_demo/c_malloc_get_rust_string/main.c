#include <stdio.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * This is windows-style api
 * if buffer NULL @return need len
 * @retrun -1 fail
 */
int get_string_api(char *buffer, uintptr_t *length);

int main(){
  size_t* len;
  get_string_api(NULL,len);
  printf("len = %ld\n", *len);

  char * buff = malloc((*len) * sizeof(char ));
  get_string_api(buff,len);
  printf("string = %s\n", buff);

  free(buff);
}