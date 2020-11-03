	.text
	.file	"out.ll"
	.globl	factorial               # -- Begin function factorial
	.p2align	4, 0x90
	.type	factorial,@function
factorial:                              # @factorial
	.cfi_startproc
# %bb.0:
	pushq	%rax
	.cfi_def_cfa_offset 16
	movl	%edi, 4(%rsp)
	xorl	%eax, %eax
	cmpl	$2, %edi
	setb	%al
	cmpl	$1, %edi
	movl	%eax, (%rsp)
	ja	.LBB0_2
# %bb.1:                                # %L0
	movl	$1, (%rsp)
	movl	(%rsp), %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.LBB0_2:                                # %L1
	.cfi_def_cfa_offset 16
	movl	4(%rsp), %edi
	decl	%edi
	movl	%edi, (%rsp)
	callq	factorial
	imull	4(%rsp), %eax
	movl	%eax, (%rsp)
	movl	(%rsp), %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	factorial, .Lfunc_end0-factorial
	.cfi_endproc
                                        # -- End function
	.globl	main                    # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:
	movl	%edi, -32(%rsp)
	movq	%rsi, -24(%rsp)
	xorl	%eax, %eax
	cmpl	$2, %edi
	setl	%al
	cmpl	$1, %edi
	movl	%eax, -12(%rsp)
	jg	.LBB1_2
# %bb.1:                                # %L0
	movl	$-1, -32(%rsp)
	movl	-32(%rsp), %eax
	retq
.LBB1_2:                                # %L1
	movq	-24(%rsp), %rax
	movq	8(%rax), %rax
	movq	%rax, -8(%rsp)
	movsbl	(%rax), %eax
	movb	%al, -25(%rsp)
	addl	$-48, %eax
	movl	%eax, -16(%rsp)
	movl	%eax, -32(%rsp)
	movl	-32(%rsp), %eax
	retq
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
	.cfi_endproc
                                        # -- End function
	.section	".note.GNU-stack","",@progbits
