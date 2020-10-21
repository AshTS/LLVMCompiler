	.text
	.file	"out.ll"
	.globl	main                    ; -- Begin function main
	.p2align	1
	.type	main,@function
main:                                   ; @main
; %bb.0:
	ldi	r24, 7
	out	4, r24
	ldi	r24, 5
	out	5, r24
LBB0_1:                                 ; %L0
                                        ; =>This Inner Loop Header: Depth=1
	rjmp	LBB0_1
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
                                        ; -- End function
	; Declaring this symbol tells the CRT that it should
	;copy all variables from program memory to RAM on startup
	.globl	__do_copy_data
	; Declaring this symbol tells the CRT that it should
	;clear the zeroed data section on startup
	.globl	__do_clear_bss
