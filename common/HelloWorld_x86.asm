section .data
  hello_world db 'Hello World!', 0x0a
  hello_world_len equ $ - hello_world
section .text
  align 4
  sys:
    int 0x80
    ret
  global _start
  _start:
    push hello_world_len
    push hello_world
    push 1
    mov eax, 4
    call sys
    push 0
    mov eax, 1
    call sys