pub mod process;

use std::path::Path;
use std::env::args;

use crate::process::*;


fn main() {
    // Path to DLL to inject
    let dll_path = "./test.dll";
    // Function to call in DLL
    let fn_name = b"main\0";
    // Get process ID of application to inject into
    let pid: u32 = args().collect::<Vec<String>>()[1].parse().unwrap();

    // Attach to process
    let process = Process::new(pid);
    // Inject DLL into process
    process.inject_dll(Path::new(dll_path), fn_name);
}
