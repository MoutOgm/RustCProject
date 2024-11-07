#include <stdio.h>
#include "lib/utils.hpp"

int main() {
    int v[1] = {rc::utils::add(4, 8)};
    rc::utils::print_int("4 + 8 = {} on print_int", v);
    rc::utils::print_str("char", "");
    bool b[3] = {0, 1, 1};
    rc::utils::print_bool("{} test {} another {}", b);
    printf("4 + 8 = %d", rc::utils::add(4, 8));
    return 0;
}