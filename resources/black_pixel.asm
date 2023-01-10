@8192
D=A
@SCREEN
D=D+A   
@max
M=D     // max = SCREEN + 8192

@colour
M=0

(FRAME)

@SCREEN
D=A

@i
M=D   // i = SCREEN

(FILL_SCREEN)

@colour
D=M

@i
A=M
M=D  // colour pixel at @i

@i
M=M+1 // increment the address pointer

D=M
@max 
D=M-D
@FILL_SCREEN
D;JGT // fill rest of screen

@KBD
D=M
@COLOUR_BLACK
D;JNE

(COLOUR_WHITE)
@colour
M=0
@FRAME
0;JMP

(COLOUR_BLACK)
@colour
M=-1

@140    // esc
D=D-A  
@FRAME
D;JNE   // loop if esc is not pressed