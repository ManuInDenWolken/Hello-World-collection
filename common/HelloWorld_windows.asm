.MODEL Small
.STACK 100h
.CONST
  HW      DB      "Hallo Welt!$"
.CODE

start:
  MOV AX,@data
  MOV DS,AX
  MOV DX, OFFSET DGROUP:HW
  MOV AH, 09H
  INT 21H
  MOV AH, 4Ch
  INT 21H
end start