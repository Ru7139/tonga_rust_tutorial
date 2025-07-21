
#include <stdio.h>
#include "useful_struct.h"

struct Boxea {
    int index_x;
    int index_y;
};

void display_boxed(struct Boxea object_box) {
    printf("alpha ---> (%d,%d)", object_box.index_x, object_box.index_y);
}

int main() {
    int a = 0;
    int * p = &a;
    *p = 1;
    printf("a = %d\n", a);

    struct Boxea alphae = { 50, 25 };
    display_boxed(alphae);


    return 0;
}
