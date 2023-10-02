core::ptr::drop_in_place<alloc::vec::Vec<i32>>:
        test    rsi, rsi
        je      .LBB0_1
        shl     rsi, 2
        mov     edx, 4
        jmp     qword ptr [rip + __rust_dealloc@GOTPCREL]
.LBB0_1:
        ret

example::index_from_start:
        push    r15
        push    r14
        push    rbx
        mov     r14, rdi
        mov     rbx, qword ptr [rdi]
        mov     rax, qword ptr [rdi + 16]
        test    rax, rax
        je      .LBB1_4
        shl     rax, 2
        xor     r15d, r15d
.LBB1_2:
        cmp     dword ptr [rbx + 4*r15], esi
        je      .LBB1_6
        inc     r15
        add     rax, -4
        jne     .LBB1_2
.LBB1_4:
        lea     rdi, [rip + .L__unnamed_1]
        lea     rdx, [rip + .L__unnamed_2]
        mov     esi, 43
        call    qword ptr [rip + core::panicking::panic@GOTPCREL]
        ud2
.LBB1_6:
        mov     rsi, qword ptr [r14 + 8]
        test    rsi, rsi
        je      .LBB1_8
        shl     rsi, 2
        mov     edx, 4
        mov     rdi, rbx
        call    qword ptr [rip + __rust_dealloc@GOTPCREL]
.LBB1_8:
        mov     rax, r15
        pop     rbx
        pop     r14
        pop     r15
        ret
        mov     r15, rax
        mov     rsi, qword ptr [r14 + 8]
        mov     rdi, rbx
        call    core::ptr::drop_in_place<alloc::vec::Vec<i32>>
        mov     rdi, r15
        call    _Unwind_Resume@PLT
        ud2

.L__unnamed_1:
        .ascii  "called `Option::unwrap()` on a `None` value"

.L__unnamed_3:
        .ascii  "/app/example.rs"

.L__unnamed_2:
        .quad   .L__unnamed_3
        .asciz  "\017\000\000\000\000\000\000\000\007\000\000\000\n\000\000"

DW.ref.rust_eh_personality:
        .quad   rust_eh_personality
