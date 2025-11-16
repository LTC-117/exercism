; Everything that comes after a semicolon (;) is a comment

; Assembler-time constants may be defined using 'equ'

EXPECTED_MINUTES equ 40

section .text

; You should implement functions in the .text section

; the global directive makes a function visible to the test files
global expected_minutes_in_oven
expected_minutes_in_oven:
    mov     eax, EXPECTED_MINUTES
    ret

global remaining_minutes_in_oven
remaining_minutes_in_oven:
    mov     eax, EXPECTED_MINUTES
    sub     eax, edi
    ret

global preparation_time_in_minutes
preparation_time_in_minutes:
    mov     eax, edi
    imul    eax, 2
    ret

global elapsed_time_in_minutes
elapsed_time_in_minutes:
    mov     eax, edi
    imul    eax, 2
    add     eax, esi
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
