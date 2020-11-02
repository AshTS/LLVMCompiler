	.text
	.file	"out.ll"
	.globl	main                    ; -- Begin function main
	.p2align	1
	.type	main,@function
main:                                   ; @main
; %bb.0:
	push	r28
	push	r29
	in	r28, 61
	in	r29, 62
	sbiw	r28, 1
	in	r0, 63
	cli
	out	62, r29
	out	63, r0
	out	61, r28
	ldi	r24, 15
	out	4, r24
	ldi	r24, -2
	out	10, r24
	ldi	r24, 0
	out	5, r24
	out	11, r24
LBB0_1:                                 ; %L0
                                        ; =>This Inner Loop Header: Depth=1
	in	r24, 9
	std	Y+1, r24
	out	5, r24
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
