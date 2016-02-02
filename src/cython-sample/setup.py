from distutils.core import setup
from Cython.Build import cythonize

setup(
    ext_modules = cythonize("sample.pyx", sources=['sample_func.c'])
)

