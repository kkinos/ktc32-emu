addi r10, r0, 80
addi r11, r0, 4

addi r12, r0, 16

jalr r1, r0, 16

beq r1, r12, 4
addi r11, r11, -1

sw r11, r10, 0