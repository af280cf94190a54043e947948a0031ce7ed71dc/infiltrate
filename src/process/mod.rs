use winapi::um::libloaderapi::{GetProcAddress, GetModuleHandleA};
use winapi::um::processthreadsapi::{OpenProcess, CreateRemoteThread, GetExitCodeThread};
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winnt::*;
use winapi::um::winbase::*;
use winapi::um::handleapi::CloseHandle;
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::NULL;

#[macro_use]
#[path = "../util/macros.rs"]
mod macros;

pub mod memory;
pub mod inject;

use sysinfo::{ProcessExt, System, SystemExt};

#[repr(C)]
pub struct Functions {
    load_library_w: *mut (),
    get_proc_address: *mut (),
    get_last_error: *mut (),
    err: u32
}

pub struct Process {
    handle: HANDLE,
    functions: Functions
}

impl Process {
    pub fn new(pid: u32) -> Self {

        unsafe {

            // Open Process with correct permissions
            let handle = OpenProcess(PROCESS_CREATE_THREAD
                    | PROCESS_QUERY_INFORMATION
                    | PROCESS_VM_OPERATION
                    | PROCESS_VM_WRITE
                    | PROCESS_VM_READ, 0, pid);
    
            
            // Load required libraries for DLL injection
            let kernel32 = GetModuleHandleA(ptr!(b"Kernel32.dll\0"));
            let load_library_w = GetProcAddress(kernel32, ptr!(b"LoadLibraryW\0")).cast();
            let get_proc_address = GetProcAddress(kernel32, ptr!(b"GetProcAddress\0")).cast();
            let get_last_error = GetProcAddress(kernel32, ptr!(b"GetLastError\0")).cast();

            println!("kernel32: {:?}", kernel32);
            println!("LoadLibraryW: {:?}", load_library_w);
            println!("GetProcAddress: {:?}", get_proc_address);
            println!("GetLastError {:?}", get_last_error);

    
            if kernel32 == NULL as _ || load_library_w == NULL as _ || get_proc_address == NULL as _ || get_last_error == NULL as _  {
                panic!("Failed to get function pointers");
            }

            Process {
                handle,
                functions: Functions {
                    load_library_w,
                    get_proc_address,
                    get_last_error,
                    err: 0
                }
            }

        }
    }
}


impl Process {
    pub fn create_remote_thread(&self, alloc: *mut std::ffi::c_void) -> HANDLE {
        let thread = unsafe {
            CreateRemoteThread(
                self.handle,
                NULL.cast(),
                0,
                std::mem::transmute(alloc),
                NULL.cast(),
                0,
                NULL.cast()
            )
        };
        if thread.is_null() {
            panic!("Couldn't create thread");
        }
        thread
    }

    pub fn wait_for_single_object(&self, thread: HANDLE) {
        if unsafe {
            WaitForSingleObject(thread, INFINITE)
        } != 0 {
            panic!("WaitForSingleObject failed");
        };
    }

    pub fn close_handle(&self, thread: HANDLE) {
        unsafe {
            CloseHandle(thread)
        };
    }

    pub fn get_exit_code(&self, thread: HANDLE) -> u32 {
        let mut exit_code: DWORD = 0;
        if unsafe {
            GetExitCodeThread(thread, &mut exit_code as _)
        } == 0 {
            panic!("GetExitCodeThread failed");
        };
        exit_code
    }
}