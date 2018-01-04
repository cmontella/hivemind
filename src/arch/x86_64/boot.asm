global start

section .text
bits 32
start:
    mov esp, stack_top

    ; Check feature support
    call check_multiboot
    call check_cpuid
    call check_long_mode

    ; print `ok` to screen
    mov dword [0xb8000], 0x2f4b2f4f
    hlt

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
.no_multiboot
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