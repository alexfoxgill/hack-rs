// multiplies the two (non-negative) values in R0 and R1, saving the result in R2

@R0
D=M
@END
D;JEQ

(LOOP)

@R1
D=M
M=M-1
@END
D;JEQ

@R0
D=M
@R2
M=D+M

@LOOP
0;JMP

(END)
