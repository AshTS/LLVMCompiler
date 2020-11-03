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
	movl	4(%rsp), %eax
	imull	(%rsp), %eax
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
	pushq	%rax
	.cfi_def_cfa_offset 16
	movl	$5, %edi
	callq	factorial
	movl	4(%rsp), %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
	.cfi_endproc
                                        # -- End function
	.section	".note.GNU-stack","",@progbits
