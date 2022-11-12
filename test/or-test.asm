addi r10, r0, 80
addi r11, r0, 4

addi r12, r0, 3

addi r1, r0, 2
addi r2, r0, 1

or r1, r2

beq r1, r12, 4
addi r11, r11, -1

sw r11, r10, 0