addi r2, r0, 5
addi r3, r0, 12
addi r7, r3, -9
mov r4, r2
mov r5, r3
or r4, r7
and r5, r4
add r5, r4
beq r5, r7, 60
slt r3, r4
beq r3, r0, 4
addi r5, r0, 1
mov r8, r2
slt r7, r8
add r5, r7
sub r5, r2
sw r5, r8, 75
lw r2, r0, 80
jal, r0, 4
addi r2, r0, 1
sw r2, r0 84