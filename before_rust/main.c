#include <stdio.h>

int main() {
    int a = 0;
    int * p = &a;
    *p = 1;
    printf("a = %d\n", a);
    return 0;
}
