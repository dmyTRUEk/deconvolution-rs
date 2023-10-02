core::ptr::drop_in_place<alloc::vec::Vec<i32>>:
        test    rsi, rsi
        je      .LBB0_1
        shl     rsi, 2
        mov     edx, 4
        jmp     qword ptr [rip + __rust_dealloc@GOTPCREL]
.LBB0_1:
        ret

example::index_from_end_by_hands:
        push    rbp
        push    r15
        push    r14
        push    r12
        push    rbx
        mov     ebp, esi
        mov     r14, rdi
        mov     rbx, qword ptr [rdi]
        mov     r15, qword ptr [rdi + 16]
        lea     rax, [4*r15]
        neg     r15
.LBB1_1:
        test    rax, rax
        je      .LBB1_7
        mov     r12d, dword ptr [rbx + rax - 4]
        add     rax, -4
        inc     r15
        cmp     r12d, ebp
        jne     .LBB1_1
        mov     rsi, qword ptr [r14 + 8]
        test    rsi, rsi
        je      .LBB1_5
        shl     rsi, 2
        mov     edx, 4
        mov     rdi, rbx
        call    qword ptr [rip + __rust_dealloc@GOTPCREL]
.LBB1_5:
        xor     eax, eax
        cmp     r12d, ebp
        setne   al
        add     rax, r15
        neg     rax
        pop     rbx
        pop     r12
        pop     r14
        pop     r15
        pop     rbp
        ret
.LBB1_7:
        lea     rdi, [rip + .L__unnamed_1]
        lea     rdx, [rip + .L__unnamed_2]
        mov     esi, 43
        call    qword ptr [rip + core::panicking::panic@GOTPCREL]
        ud2
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
        .asciz  "\017\000\000\000\000\000\000\000\017\000\000\000\n\000\000"

DW.ref.rust_eh_personality:
        .quad   rust_eh_personality
