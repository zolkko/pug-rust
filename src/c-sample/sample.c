#include <Python.h>

static int __attribute__ ((noinline)) 
cadd_two(int a, int b)
{
    int acc = 0;
    for (int i = 0; i < 1000; i++) acc += a + b;
    return acc;
}

static PyObject *
add_two(PyObject * self, PyObject * args)
{
    int a, b, acc = 0;
    if (!PyArg_ParseTuple(args, "ii", &a, &b)) {
        PyErr_SetNone(PyExc_ValueError);
        return NULL;
    }
    return Py_BuildValue("i", cadd_two(a, b));
}

static PyMethodDef SampleMethods[] = {
    {"add_two", add_two, METH_VARARGS, ""},
    {NULL, NULL, 0, NULL}
};

void initsample(void)
{
    printf("sample 2\n");
    PyObject * m = Py_InitModule("sample", SampleMethods);
}

