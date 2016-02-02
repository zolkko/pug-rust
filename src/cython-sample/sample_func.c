
#include <stdint.h>
#include "sample.h"


int32_t cadd_two(int32_t a, int32_t b)
{
    int32_t acc = 0;
    for (int i = 0; i < 1000; i++) {
        acc += a + b;
    }
    return acc;
}

