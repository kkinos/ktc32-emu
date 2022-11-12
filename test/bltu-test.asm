addi r10, r0, 80
addi r11, r0, 4

addi r12, r0, 1

addi r1, r0, 1
addi r2, r0, 1
addi r3, r0, -1

bltu r2, r3, 4
addi r1, r1, 1

beq r1, r12, 4
addi r11, r11, -1

sw r11, r10, 0