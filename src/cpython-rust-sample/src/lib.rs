// #![feature(plugin)]

#[macro_use]
extern crate cpython;


use cpython::{PyObject, PyResult, Python, PyTuple, PyDict, ToPyObject, PythonObject};


fn add_two(py: Python, a: u32, b: u32) -> PyResult<u32> {

    let mut acc:u32 = 0;

    for _ in 0..1000 { 
        acc += a + b;
    }

    Ok(acc)
}

py_module_initializer!(example, initexample, PyInit_example, |py, module| {
    module.add(py, "__doc__", "This module is implemented in Rust.")?;
    module.add(py, "add_two", py_fn!(py, add_two(a: u32, b: u32)))?;
    Ok(())
});

