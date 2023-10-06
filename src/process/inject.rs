use crate::process::*;

use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::LPCVOID;

const SHELLCODE: &[u8] = include_bytes!("../../asm/x86.bin");

impl Process {
    pub fn inject_dll(&self, dll: &std::path::Path, fname: &[u8]) {

        let dll_path: Vec<u16> = dll
            .canonicalize()
            .unwrap()
            .into_os_string()
            .encode_wide()
            .collect();

        println!("Loading DLL: {:?}", String::from_utf16(&dll_path).unwrap());

        // Calculate total memory space required for injection and allocate it
        let fns_len = std::mem::size_of::<Functions>();
        let dll_len = (dll_path.len() + 1) * 2;
        let fn_len = fname.len() + 1;

        let size = SHELLCODE.len() + fns_len + dll_len + fn_len;

        let alloc = self.virtual_alloc_ex(size);


        println!("Allocating {:?} Bytes From {:?}", size, alloc);

        // Write the injection shellcode to process
        self.write_process_memory(
            alloc,
            SHELLCODE.as_ptr().cast(),
            SHELLCODE.len(),
        );
        println!("Wrote Shellcode at {:?}", alloc);

        let fns: LPCVOID = &self.functions as *const _ as _;
        println!("{:?}", fns);
        
        /* 
        Write the 3 functions required to the process at position after shellcode
        asm/x86.s ; ln 60 - ln 67
        asm/x64.s ; ln 56 - ln 61
        */
        self.write_process_memory(
            unsafe { alloc.add(SHELLCODE.len()) },
            fns,
            fns_len,
        );
        println!("Wrote Functions at {:?}", unsafe { alloc.add(SHELLCODE.len()) });
        
        /* 
        Write DLL Path after the functions in shellcode
        asm/x86.s ; ln 72
        asm/x64.s ; ln 64
        */
        self.write_process_memory(
            unsafe { alloc.add(SHELLCODE.len() + fns_len) },
            dll_path.as_ptr().cast(),
            dll_len - 2,
        );
        println!("Wrote DLL Path at {:?}", unsafe { alloc.add(SHELLCODE.len() + fns_len) });

        // Write name of function to call at end of shellcode
        self.write_process_memory(
            unsafe { alloc.add(SHELLCODE.len() + fns_len + dll_len) },
            fname.as_ptr().cast(),
            fn_len - 1,
        );
        println!("Wrote Function Name at {:?}", unsafe { alloc.add(SHELLCODE.len() + fns_len + dll_len) });

        // Create thread to call specified function
        let thread = self.create_remote_thread(alloc);
        println!("Created Thread at {:?}", thread);
        self.wait_for_single_object(thread);
        println!("Thread Finished");

        // Ensure process memory reads correctly
        let error = self.read_process_memory(unsafe { alloc.add(SHELLCODE.len() + fns_len - std::mem::size_of::<usize>()) as _ }, 4);
        println!("ReadProcess Error: {:x}", error);

        // done, cleanup

        let error = self.get_exit_code(thread);
        println!("Thread Exit Code: {:x}", error);

        self.virtual_free_ex(alloc.cast(), size);
        self.close_handle(thread);
    }
}