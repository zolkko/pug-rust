#![feature(slice_patterns)]
#![feature(plugin)]
#![plugin(interpolate_idents)]

#[macro_use]
extern crate cpython;


use cpython::{PyObject, PyResult, Python, PyTuple, PyDict, ToPyObject, PythonObject};


fn add_two(py: Python, args: &PyTuple, _: Option<&PyDict>) -> PyResult<PyObject> {
    match args.as_slice() {
        [ref a_obj, ref b_obj] => {
            let a = a_obj.extract::<i32>(py).unwrap();
            let b = b_obj.extract::<i32>(py).unwrap();
            let mut acc:i32 = 0;

            for _ in 0..1000 { 
                acc += a + b;
            }

            Ok(acc.to_py_object(py).into_object())
        },
        _ => Ok(py.None())
    }
}

py_module_initializer!(example, |py, module| {
    try!(module.add(py, "add_two", py_fn!(add_two)));
    Ok(())
});

