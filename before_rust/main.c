#include <stdio.h>
#include "useful_struct.h"

struct Boxea {
    int index_x;
    int index_y;
};

int main() {
    int a = 0;
    int * p = &a;
    *p = 1;
    printf("a = %d\n", a);

    struct Boxea alphae = { 50, 25 };
    printf("alpha ---> (%d,%d)", alphae.index_x, alphae.index_y);

    return 0;
}
