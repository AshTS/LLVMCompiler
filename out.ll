define i32 @factorial(i32 %value)
{
    %V0 = alloca i32, align 4
    %V1 = alloca i32, align 4
    store i32 %value, i32* %V1

; clt    %R2 (u32)      %value (u32)   2 (u32)        
    %V4 = load i32, i32* %V1, align 4
    %V2 = icmp ult i32 %V4, 2
    %V3 = zext i1 %V2 to i32
    store i32 %V3, i32* %V0

; bne    %R2 (u32)      0 (u32)        L0             L1             
    %V6 = load i32, i32* %V0, align 4
    %V5 = icmp ne i32 %V6, 0
    br i1 %V5, label %L0, label %L1
    br label %L0

  L0:

; mov    %R2 (u32)      1 (u32)        
    store i32 1, i32* %V0

; jmp    L2             
    br label %L2
    br label %L1

  L1:

; sub    %R2 (u32)      %value (u32)   1 (u32)        
    %V8 = load i32, i32* %V1, align 4
    %V7 = sub i32 %V8, 1
    store i32 %V7, i32* %V0

; push   %R2 (u32)      
    %V9 = load i32, i32* %V0, align 4

; call   %R2 (u32)      factorial      
    %V10 = call i32 @factorial(i32 %V9)
    store i32 %V10, i32* %V0

; mul    %R2 (u32)      %value (u32)   %R2 (u32)      
    %V12 = load i32, i32* %V1, align 4
    %V13 = load i32, i32* %V0, align 4
    %V11 = mul i32 %V12, %V13
    store i32 %V11, i32* %V0
    br label %L2

  L2:

; ret    %R2 (u32)      
    %V14 = load i32, i32* %V0, align 4
    ret i32 %V14
}
define i32 @main(i32 %argc, i8** %argv)
{
    %V0 = alloca i32, align 4
    %V1 = alloca i32, align 4
    store i32 %argc, i32* %V1
    %V2 = alloca i8*, align 8
    %V3 = alloca i8**, align 8
    store i8** %argv, i8*** %V3
    %V4 = alloca i8, align 1
    %V5 = alloca i32, align 4

; clt    %R1 (i32)      %argc (i32)    2 (i32)        
    %V8 = load i32, i32* %V1, align 4
    %V6 = icmp slt i32 %V8, 2
    %V7 = zext i1 %V6 to i32
    store i32 %V7, i32* %V0

; bne    %R1 (i32)      0 (i32)        L0             L1             
    %V10 = load i32, i32* %V0, align 4
    %V9 = icmp ne i32 %V10, 0
    br i1 %V9, label %L0, label %L1
    br label %L0

  L0:

; mov    %argc (i32)    -1 (i32)       
    store i32 -1, i32* %V1

; jmp    exit           
    br label %exit
    br label %L1

  L1:

; array  %R3 (i8*)      %argv (i8**)   1 (u64)        
    %V13 = load i8**, i8*** %V3, align 8
    %V11 = getelementptr i8*, i8** %V13, i64 1
    %V12 = load i8*, i8** %V11, align 8
    store i8* %V12, i8** %V2

; array  %R4 (i8)       %R3 (i8*)      0 (u64)        
    %V16 = load i8*, i8** %V2, align 8
    %V14 = getelementptr i8, i8* %V16, i64 0
    %V15 = load i8, i8* %V14, align 1
    store i8 %V15, i8* %V4

; cast   %R5 (i32)      %R4 (i8)       
    %V17 = load i8, i8* %V4, align 1
    %V18 = sext i8 %V17 to i32
    store i32 %V18, i32* %V5

; sub    %R5 (i32)      %R5 (i32)      48 (i32)       
    %V20 = load i32, i32* %V5, align 4
    %V19 = sub i32 %V20, 48
    store i32 %V19, i32* %V5

; mov    %argc (i32)    %R5 (i32)      
    %V21 = load i32, i32* %V5, align 4
    store i32 %V21, i32* %V1
    br label %exit

  exit:

; ret    %argc (i32)    
    %V22 = load i32, i32* %V1, align 4
    ret i32 %V22
}
