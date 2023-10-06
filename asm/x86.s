BITS 32

entry: ; main entry point
    call set_si
    sub esi, 5 ; subtract 5 from source index (size of call)

    ; LoadLibraryW(DllName);
    lea edi, [esi + DllName] ; Dereference DllName and copy into edi
                             ; edi becomes same value as DllName
    push edi ; push DllName onto stack
    call [esi + LoadLibraryW] ; Call LoadLibraryW
    test eax, eax ; if LoadLibraryW returned 0, then we failed
    jz error ; jump to error if LoadLibraryW returned 0

    ; HMODULE for DLL now stored in eax

    call go_fn ; move edi to name of function to call
    cmp byte [edi], 0 
    jz done_nofn ; if edi is null (no function name) then end

    ; GetProcAddress(eax, edi);
    ; eax is HMODULE, edi is function name
    push edi
    push eax
    call [esi + GetProcAddress]
    test eax, eax
    jz error

    call eax ; call address of function
    jmp done


error:
    call [esi+GetLastError] ; get last error
    mov [esi+Error], eax ; save error code

done_nofn:
    xor eax, eax ; clear eax and end here

done:
    ret 4

ALIGN 4
set_si:
    mov esi, [esp] ; move stack pointer into source index
    ret

ALIGN 4
go_fn:
    cmp word [edi], 0 ; check if current edi value is null
    lea edi, [edi+2] ; move to next char in memory
    jnz go_fn ; wait until edi is null (start of next string)
    ret

ALIGN 4
section .bss

; reserve memory space for function addresses

LoadLibraryW:
resd 1

GetProcAddress:
resd 1

GetLastError:
resd 1

Error:
resd 1

DllName:

