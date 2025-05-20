// Main entry point for the core-rs library

// define the alias module
mod alias;

use alias::{AliasFlags, AliasManager};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::OnceLock;

static ALIAS_MANAGER: OnceLock<AliasManager> = OnceLock::new();

fn get_manager() -> &'static AliasManager {
    ALIAS_MANAGER.get_or_init(AliasManager::new)
}

#[unsafe(no_mangle)]
/// # Safety
/// This function is unsafe because it takes raw pointers as arguments.
pub unsafe extern "C" fn alias_add(name: *const c_char, value: *const c_char) {
    if name.is_null() || value.is_null() {
        return;
    }
    let cname = unsafe { CStr::from_ptr(name) };
    let cvalue = unsafe { CStr::from_ptr(value) };
    if let (Ok(name), Ok(value)) = (cname.to_str(), cvalue.to_str()) {
        get_manager().add_alias(name, value);
    }
}

#[unsafe(no_mangle)]
/// # Safety
/// This function is unsafe because it takes raw pointers as arguments.
pub unsafe extern "C" fn alias_get_value(name: *const c_char) -> *mut c_char {
    if name.is_null() {
        return std::ptr::null_mut();
    }
    let cname = unsafe { CStr::from_ptr(name) };
    if let Ok(name) = cname.to_str() {
        if let Some(val) = get_manager().get_alias_value(name) {
            if let Ok(cstr) = CString::new(val) {
                return cstr.into_raw();
            }
        }
    }
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
/// # Safety
/// This function is unsafe because it takes raw pointers as arguments.
pub unsafe extern "C" fn alias_remove(name: *const c_char) -> bool {
    if name.is_null() {
        return false;
    }
    let cname = unsafe { CStr::from_ptr(name) };
    if let Ok(name) = cname.to_str() {
        return get_manager().remove_alias(name);
    }
    false
}

#[unsafe(no_mangle)]
pub extern "C" fn alias_delete_all() {
    get_manager().delete_all_aliases();
}

#[unsafe(no_mangle)]
pub extern "C" fn alias_list() -> *mut *mut c_char {
    let aliases = get_manager().all_aliases();
    let mut c_vec: Vec<*mut c_char> = Vec::with_capacity(aliases.len() + 1);
    for alias in aliases {
        let pair = format!("{}={}", alias.name, alias.value);
        if let Ok(cstr) = CString::new(pair) {
            c_vec.push(cstr.into_raw());
        }
    }
    c_vec.push(std::ptr::null_mut()); // NULL-terminate
    let ptr = c_vec.as_mut_ptr();
    std::mem::forget(c_vec); // Prevent Rust from freeing the memory
    ptr
}

#[unsafe(no_mangle)]
/// # Safety
/// This function is unsafe because it takes raw pointers as arguments.
pub unsafe extern "C" fn alias_list_free(list: *mut *mut c_char) {
    unsafe {
        if list.is_null() {
            return;
        }
        let mut i = 0;
        loop {
            let ptr = *list.add(i);
            if ptr.is_null() {
                break;
            }
            let _ = CString::from_raw(ptr);
            i += 1;
        }
        // Free the array itself
        let _ = Vec::from_raw_parts(list, i + 1, i + 1);
    }
}
