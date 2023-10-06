use crate::process::Process;

use winapi::um::memoryapi::{
    WriteProcessMemory,
    ReadProcessMemory,
    VirtualAllocEx,
    VirtualFreeEx
};

use winapi::um::errhandlingapi::GetLastError;
use winapi::shared::minwindef::*;
use winapi::shared::ntdef::*;
use winapi::um::winnt::*;

// memoryapi related functions
impl Process {
    pub fn virtual_alloc_ex(&self, size: usize) -> *mut std::ffi::c_void {
        let alloc = unsafe { VirtualAllocEx(
            self.handle, 
            NULL, 
            size, 
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE
        ) }.cast::<std::ffi::c_void>();
        if alloc.is_null() {
            panic!("VirtualAllocEx failed");
        }
        alloc
    }

    pub fn write_process_memory(&self, base_addr: LPVOID, buffer: LPCVOID, size: usize) {
        if unsafe {
            WriteProcessMemory(
                self.handle,
                base_addr,
                buffer,
                size,
                NULL as _
            )
        } == 0 {
            panic!("WriteProcessMemory failed");
        }
    }

    pub fn read_process_memory(&self, base_addr: LPCVOID, size: usize) -> DWORD {
        let mut error: DWORD = 0;
        if unsafe {
            ReadProcessMemory(
                self.handle,
                base_addr,
                &mut error as *const _ as _,
                size,
                NULL as _
            )
        } == 0 {
            println!("{:?} {:x}", unsafe { GetLastError() }, error);
            panic!("ReadProcessMemory failed");
        }
        error
    }

    pub fn virtual_free_ex(&self, base_addr: LPVOID, size: usize) {
        unsafe {
            VirtualFreeEx(
                self.handle,
                base_addr,
                size,
                MEM_RELEASE
            )
        };
    }
}