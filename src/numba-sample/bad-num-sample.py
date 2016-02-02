from numba import float32, int32, jit
import cProfile
from dis import dis

def summ(a, b):
    return a + b

# @jit(int32(int32, int32), nopython=True, nogil=True)
@jit(int32(int32, int32), nogil=True)
def add_two(a, b):
    acc = 0
    i = 0
    while i < 1000:
        acc += summ(a, b)
        i += 1
    return acc


def add_two_wrap(a, b):
    return add_two(a, b)


def add_two2(a, b):
    acc = 0
    i = 0
    while i < 1000:
        acc += a + b
        i += 1
    return acc


def test():
    num = 100
    print add_two_wrap(num, num + 1)
    print add_two2(num, num + 1)


if __name__ == '__main__':
       cProfile.run('test()')
       print '%s' % add_two.inspect_asm().values()[0].decode('string_escape')
       print dis(add_two2)

