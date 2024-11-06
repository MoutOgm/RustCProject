#include <stdio.h>
#include "lib/rclib.hpp"

class a {
    public :
    int b, c;
    a (int b, int c) {
        this->b = b;
        this->c = c;
    }
    inline int add() {
        return rc::add(this->b, this->c);
    }
};

int main() {
    a *v = new a(4, 5);
    printf("Valeur totale de la classe 'a' : %d", rc::value());
    return 0;
}
