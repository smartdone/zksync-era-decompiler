
Assembly:
	.text
	.file	"demo0.sol:demo0"
	.globl	__entry
__entry:
.func_begin0:
	add	128, r0, r3
	stm.h	64, r3
	and!	1, r2, r0
	jump.ne	@.BB0_1
	shr.s	96, r1, r2
	and	code[@CPI0_0], r2, r2
	sub.s!	4, r2, r0
	jump.lt	@.BB0_16
	ldp	r1, r3
	shr.s	224, r3, r3
	sub.s!	code[@CPI0_2], r3, r0
	jump.eq	@.BB0_12
	sub.s!	code[@CPI0_3], r3, r0
	jump.eq	@.BB0_10
	sub.s!	code[@CPI0_4], r3, r0
	jump.ne	@.BB0_16
	sub.s!	36, r2, r0
	jump.lt	@.BB0_16
	ldvl	r2
	sub!	r2, r0, r0
	jump.ne	@.BB0_16
	addp.s	4, r1, r1
	ldp	r1, r1
.BB0_15:
	sts	r0, r1
	add	r0, r0, r1
	retl	@DEFAULT_FAR_RETURN
.BB0_1:
	ldvl	r1
	sub!	r1, r0, r0
	jump.ne	@.BB0_16
	add	32, r0, r1
	stm.ah	256, r1
	stm.ah	288, r0
	add	code[@CPI0_1], r0, r1
	retl	@DEFAULT_FAR_RETURN
.BB0_10:
	ldvl	r1
	sub!	r1, r0, r0
	jump.ne	@.BB0_16
	lds	r0, r1
	stm.h	128, r1
	add	code[@CPI0_10], r0, r1
	retl	@DEFAULT_FAR_RETURN
.BB0_12:
	ldvl	r1
	sub!	r1, r0, r0
	jump.ne	@.BB0_16
	par	r1
	sub!	r1, r0, r0
	jump.ne	@.BB0_17
	lds	r0, r1
	add!	1, r1, r1
	jump.ne	@.BB0_15
	add	code[@CPI0_8], r0, r1
	stm.h	0, r1
	add	17, r0, r1
	stm.h	4, r1
	add	code[@CPI0_9], r0, r1
	revl	@DEFAULT_FAR_REVERT
.BB0_16:
	add	r0, r0, r1
	revl	@DEFAULT_FAR_REVERT
.BB0_17:
	add	code[@CPI0_5], r0, r1
	stm.h	128, r1
	add	32, r0, r1
	stm.h	132, r1
	add	4, r0, r1
	stm.h	164, r1
	add	code[@CPI0_6], r0, r1
	stm.h	196, r1
	add	code[@CPI0_7], r0, r1
	revl	@DEFAULT_FAR_REVERT
.func_end0:

	.rodata
CPI0_0:
	.cell	4294967295
CPI0_1:
	.cell	53919893334301279589334030174039261352344891250716429051063678533632
CPI0_2:
	.cell	3500007562
CPI0_3:
	.cell	2206332298
CPI0_4:
	.cell	1068876235
CPI0_5:
	.cell	3963877391197344453575983046348115674221700746820753546331534351508065746944
CPI0_6:
	.cell	54490394922520587650080162453656890713529578349178079819725652483204479713280
CPI0_7:
	.cell	7922816253787617000789217640448
CPI0_8:
	.cell	35408467139433450592217433187231851964531694900788300625387963629091585785856
CPI0_9:
	.cell	2852213850513516153367582212096
CPI0_10:
	.cell	2535301202817642044428229017600
	.text
DEFAULT_UNWIND:
	pncl	@DEFAULT_UNWIND
DEFAULT_FAR_RETURN:
	retl	@DEFAULT_FAR_RETURN
DEFAULT_FAR_REVERT:
	revl	@DEFAULT_FAR_REVERT

