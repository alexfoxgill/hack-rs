@SCREEN // set address register to top-left pixel
M=1     // blacken pixel

(LOOP)

@KBD
D=M

@140 // esc
D=D-A
@LOOP
D;JNE