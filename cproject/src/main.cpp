#include <stdio.h>
#include "lib/utils.h"


int main() {
    rc::utils::T t = rc::utils::T {1, 2};
    printf("add: %d\n", t.add());
    return 0;
}