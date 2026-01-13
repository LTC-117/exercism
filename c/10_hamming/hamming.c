#include "hamming.h"

#include <string.h>


int compute(const char *lhs, const char *rhs)
{
    int distance = 0;

    unsigned int size_lhs = strlen(lhs);
    unsigned int size_rhs = strlen(rhs);
    
    if (size_lhs - size_rhs != 0) {
        return -1;
    }

    for (unsigned int i = 0; i < size_lhs; i++) {
        if (lhs[i] != rhs[i]) {
            distance++;
        }
    }

    return distance;
}
