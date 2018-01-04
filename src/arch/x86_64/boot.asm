global start
extern long_mode_start

section .text
bits 32
start:
    mov esp, stack_top

    ; Check feature support
    call check_multiboot
    call check_cpuid
    call check_long_mode

    ; Set up paging
    call set_up_page_tables
    call enable_paging

    ; load the 64-bit GDT
    lgdt [gdt64.pointer]

    ; load long mode
    jmp gdt64.code:long_mode_start

    ; print `OK` to screen
    mov dword [0xb8000], 0x2f4b2f4f
    hlt

; Link P4 first entry to the P3 table, and the P3 first entry to the P2 table

set_up_page_tables:
    ; map first P4 entry to P3 table
    mov eax, p3_table
    or eax, 0b11 ; present + writable
    mov [p4_table], eax

    ; map first P3 entry to P2 table
    mov eax, p2_table
    or eax, 0b11 ; present + writable
    mov [p3_table], eax

    ; Identity map the first gigabyte of kernel
    ; physical address -> virtual 
    ; We do this in a loop, going through each one until
    ; we reach 512
    mov ecx, 0  ; a counter for our loop

.map_p2_table:
    ; map ecx-th P2 entry to a page that starts at address 2MiB*ecx
    mov eax, 0x200000               ; 2MiB
    mul ecx                         ; start address of ecx-th page
    or eax, 0b10000011              ; present + writable + huge
    mov [p2_table + ecx * 8], eax   ; map ecx-th entry

    inc ecx                         ; increase counter
    cmp ecx, 512                    ; if counter is 512, the P2 table is full
    jne .map_p2_table               ; map the next entry

    ret

; eable paging
; 1 - write address of p4 table to CR3 register
; 2 - enable Physical Address Extension (PAE)
; 3 - Set long mode bit in EFER register
; 4 - Finally enable paging

enable_paging:
    ; 1 - load P4 to CR3 register to access P4 table
    mov eax, p4_table
    mov cr3, eax

    ; 2 - enable PAE-flag in CR4
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; 3 - set long mode bit in EFER Model Specific Register (MSR)
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; 4 - enable paging in the cr0 register
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ret    

; -------- Error Check Functions --------

; Prints an error code and halts the processor
; Error Codes:
; 0 - No multiboot support
; 1 - CPUID not supported
; 2 - Long mode not supported

error:
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8000], 0x4f3a4f52
    mov dword [0xb8000], 0x4f204f20
    mov byte  [0xb800a], al
    hlt

; -- Multiboot Compliance --
; Check that the kernel was really loaded into a multiboot
; compliant bootloader. Check eax register for magic number
; 0x36d76289

check_multiboot:
    cmp eax, 0x36d76289
    jne .no_multiboot
    ret
.no_multiboot:
    mov al, "0"
    jmp error

; -- CPUID Support --
; Check that the cpu supports the CPUID instruction
; From OSDev Wiki: http://wiki.osdev.org/Setting_Up_Long_Mode#Detection_of_CPUID

check_cpuid:
    ; Check if CPUID is supported by attempting to flip the ID bit (bit 21)
    ; in the FLAGS register. If we can flip it, CPUID is available.

    ; Copy FLAGS in to EAX via stack
    pushfd
    pop eax

    ; Copy to ECX as well for comparing later on
    mov ecx, eax

    ; Flip the ID bit
    xor eax, 1 << 21

    ; Copy EAX to FLAGS via the stack
    push eax
    popfd

    ; Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
    pushfd
    pop eax

    ; Restore FLAGS from the old version stored in ECX (i.e. flipping the
    ; ID bit back if it was ever flipped).
    push ecx
    popfd

    ; Compare EAX and ECX. If they are equal then that means the bit
    ; wasn't flipped, and CPUID isn't supported.
    cmp eax, ecx
    je .no_cpuid
    ret
.no_cpuid:
    mov al, "1"
    jmp error

; -- Long Mode Support --
; Use CPUID to check if the processor supports long mode.
; From OSDev Wiki http://wiki.osdev.org/Setting_Up_Long_Mode#x86_or_x86-64

check_long_mode:
    ; test if extended processor info in available
    mov eax, 0x80000000    ; implicit argument for cpuid
    cpuid                  ; get highest supported argument
    cmp eax, 0x80000001    ; it needs to be at least 0x80000001
    jb .no_long_mode       ; if it's less, the CPU is too old for long mode

    ; use extended info to test if long mode is available
    mov eax, 0x80000001    ; argument for extended processor info
    cpuid                  ; returns various feature bits in ecx and edx
    test edx, 1 << 29      ; test if the LM-bit is set in the D-register
    jz .no_long_mode       ; If it's not set, there is no long mode
    ret
.no_long_mode:
    mov al, "2"
    jmp error

; Create a stack
section .bss
align 4096
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096
stack_bottom:
    resb 64
stack_top:

; Create a Global Descriptor Table (GDT)
section .rodata
gdt64:
    dq 0 ; first entry is zero
.code: equ $ - gdt64
    dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53)
.pointer:
    dw $ - gdt64 - 1
    dq gdt64