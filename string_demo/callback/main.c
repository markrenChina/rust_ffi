#include <stdio.h>

typedef void (*Callback)(const char*);

void rust_call_c(Callback callback);

void callback(const char* string) {
    printf("printed from C: %s \n", string);
}

int main() {
    rust_call_c(callback);
    return 0;
}