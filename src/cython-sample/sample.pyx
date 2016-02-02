cdef extern from 'stdint.h':
    ctypedef int int32_t

cdef extern from "sample_func.h":
    int32_t cadd_two(int32_t, int32_t)

def add_two2(a, b):
    return cadd_two(a, b)


#def add_two(a, b):
#    i = acc = 0
#    while i < 1000:
#        acc += a + b
#    return acc

