#include <stdio.h>
#include "lib/utils.hpp"
#include "lib/error_manager.hpp"

int main() {
    rc::error_manager::new_manager();
    return 0;
}