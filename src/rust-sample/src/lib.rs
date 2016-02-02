#[macro_use]
extern crate lazy_static;

const PYTHON_API_VERSION: usize = 1013;

static II_ARGS: &'static [i8] = &[0x69, 0x69, 0];
static I_ARGS: &'static [i8] = &[0x68, 0];
static ADD_TWO: &'static [i8] = &[0x61, 0x64, 0x64, 0x5F, 0x74, 0x77, 0x6F, 0];
static SAMPLE: &'static [i8] = &[0x73, 0x61, 0x6D, 0x70, 0x6C, 0x65, 0];

type PyCFunction = unsafe extern "C" fn (slf: *mut isize, args: *mut isize) -> *mut isize;

#[repr(C)]
struct PyMethodDef {
    pub ml_name: *const i8,
    pub ml_meth: Option<PyCFunction>,
    pub ml_flags: i32,
    pub ml_doc: *const i8,
}
unsafe impl Sync for PyMethodDef { }

#[link(name = "python2.7")]
extern {
    fn Py_InitModule4_64(name: *const i8,
        methods: *const PyMethodDef,
        doc: *const i8, s: isize, apiver: usize) -> *mut isize;
    fn PyArg_ParseTuple(arg1: *mut isize, arg2: *const i8, ...) -> isize;
    fn Py_BuildValue(arg1: *const i8, ...) -> *mut isize;
}

#[allow(unused_variables)]
unsafe extern "C" fn add_two(slf: *mut isize, args: *mut isize) -> *mut isize {
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let res = PyArg_ParseTuple(args,
        &II_ARGS[0] as *const _,
        &a as *const i32, &b as *const i32);
    if res == 0 {
        return 0 as *mut _;
    }
    let mut acc: i32 = 0;
    for i in 0..1000 { acc += a + b; }
    //let acc: i32 = (0..).take(100).map(|_| a + b).fold(0, |acc, x| acc + x);
    Py_BuildValue(&I_ARGS[0] as *const _, acc)
}

lazy_static! {
    static ref METHODS: Vec<PyMethodDef> = { vec![
        PyMethodDef {
            ml_name: &ADD_TWO[0] as *const _,
            ml_meth: Some(add_two),
            ml_flags: 0,
            ml_doc: 0 as *const i8
        },
        PyMethodDef {
            ml_name: 0 as *const i8,
            ml_meth: None,
            ml_flags: 0,
            ml_doc: 0 as *const i8
        }
    ] };
}

#[no_mangle]
pub extern fn initsample() {
    unsafe {
        Py_InitModule4_64(&SAMPLE[0] as *const _,
            &METHODS[0] as *const _,
            0 as *const _,
            0,
            PYTHON_API_VERSION);
    };
}

