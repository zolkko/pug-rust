import cProfile
from itertools import product


LST = list(map(''.join, product('abc', repeat=17)))




def foo():
    return map(str.upper, LST)

def bar():
    res = []
    for i in LST:
        res.append(i.upper())
    return res

def baz():
    return [i.upper() for i in LST]

def test():
    foo()
    bar()
    baz()


cProfile.run('test()')
#from dis import dis
#
#dis(bar)
#print '-----------'
#dis(baz)

