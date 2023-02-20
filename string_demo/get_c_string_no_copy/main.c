#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

extern uint32_t
print_c_string(const char *str);

int main(void) {
  print_c_string("göes to élevên");
}