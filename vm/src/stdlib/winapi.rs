use std::io;
use winapi::um::handleapi;
use winapi::um::winnt::HANDLE;

use super::os::convert_io_error;
use crate::pyobject::{PyObjectRef, PyResult};
use crate::VirtualMachine;

fn winapi_CloseHandle(handle: usize, vm: &VirtualMachine) -> PyResult<()> {
    let res = unsafe { handleapi::CloseHandle(handle as HANDLE) };
    if res == 0 {
        Err(convert_io_error(vm, io::Error::last_os_error()))
    } else {
        Ok(())
    }
}

pub fn make_module(vm: &VirtualMachine) -> PyObjectRef {
    let ctx = &vm.ctx;
    py_module!(vm, "_winapi", {
        "CloseHandle" => ctx.new_rustfunc(winapi_CloseHandle),
    })
}
