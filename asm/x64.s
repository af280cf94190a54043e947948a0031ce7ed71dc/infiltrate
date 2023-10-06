BITS 64

entry:
    lea rcx, [rel DllName] ; set rcx to DllName 
                           ; rel gives rip + DllName
    push rcx               ; push rcx to stack
    sub rsp, 32            ; 32 bytes of shadow space on stack as this may be used
    ;; |--------------| ;;
    ;; | rcx          | ;; 
    ;; | 32 bytes     | ;; <-
    ;; |--------------| ;;

    call [rel LoadLibraryW] ; call LoadLibraryW
    ;; |--------------| ;;
    ;; | rcx          | ;; 
    ;; | 32 bytes     | ;; 
    ;; | ret address  | ;; <- stack when running LoadLibraryW
    ;; |--------------| ;;
    mov rdx, [rsp + 32] ; move DllName into rdx
    test rax, rax
    jz error

    mov rcx, rax ; move HANDLE into rcx
    call go_fn

    cmp byte [rdx], 0
    jz done_nofn

    call [rel GetProcAddress]
    test rax, rax
    jz error

    call rax
    jmp done

done:
    add rsp, 40 ; remove shadow space (32 bytes + rcx)
    ret

done_nofn:
    xor rax, rax

error:
    call [rel GetLastError]
    mov [rel Error], eax

ALIGN 4
go_fn:
    cmp word [rdx], 0 
    lea rdx, [rdx + 2]
    jnz go_fn
    ret

ALIGN 8
section .bss
LoadLibraryW:
resq 1
GetProcAddress:
resq 1
GetLastError:
resq 1
Error:
resq 1
DllName:

