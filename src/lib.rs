use std::ffi::CStr;
use std::os::raw::c_char;

mod core;
mod layout;

use core::*;

#[unsafe(no_mangle)]
pub extern "C" fn frameflow_get_node(system: *mut System, id: NodeId) -> *mut Node {
    let sys = unsafe { &mut *system };
    if let Some(node) = get_node(sys, id) {
        node as *mut Node
    } else {
        std::ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn frameflow_init_system() -> *mut System {
    // move to core.rs
    let system = Box::new(System::new());
    Box::into_raw(system)
}

#[unsafe(no_mangle)]
pub extern "C" fn frameflow_destroy_system(system: *mut System) {
    if system.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(system));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn frameflow_compute_layout(system: *mut System, parent: NodeId) {
    // Placeholder for layout computation logic
}

#[unsafe(no_mangle)]
pub extern "C" fn frameflow_is_valid_node(system: *mut System, node_id: NodeId) -> bool {
    let sys = unsafe { &mut *system };
    is_valid(sys, node_id)
}

#[unsafe(no_mangle)]
pub extern "C" fn frameflow_add_generic(system: *mut System, parent: NodeId) -> NodeId {
    let sys = unsafe { &mut *system };
    if let Ok(id) = add_generic(sys, parent) {
        id
    } else {
        NodeId::invalid()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn frameflow_add_center(system: *mut System, parent: NodeId) -> NodeId {
    NodeId::new(usize::MAX, 0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_greet(name: *const c_char) {
    unsafe {
        let s = CStr::from_ptr(name).to_string_lossy();
        println!("RUST SAYS: hello, {s}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
