define i32 @factorial(i32 %value)
{
    %V0 = alloca i32, align 4
    %V1 = alloca i32, align 4
    store i32 %value, i32* %V1
    %V4 = load i32, i32* %V1, align 4
    %V2 = icmp ult i32 %V4, 2
    %V3 = zext i1 %V2 to i32
    store i32 %V3, i32* %V0
    %V6 = load i32, i32* %V0, align 4
    %V5 = icmp ne i32 %V6, 0
    br i1 %V5, label %L0, label %L1
    br label %L0

  L0:
    store i32 1, i32* %V0
    br label %L2
    br label %L1

  L1:
    %V8 = load i32, i32* %V1, align 4
    %V7 = sub i32 %V8, 1
    store i32 %V7, i32* %V0
    %V9 = load i32, i32* %V0, align 4
    %V10 = call i32 @factorial(i32 %V9)
    %V12 = load i32, i32* %V1, align 4
    %V13 = load i32, i32* %V0, align 4
    %V11 = mul i32 %V12, %V13
    store i32 %V11, i32* %V0
    br label %L2

  L2:
    %V14 = load i32, i32* %V0, align 4
    ret i32 %V14
}
define i32 @main()
{
    %V0 = alloca i32, align 4
    %V1 = call i32 @factorial(i32 5)
    %V2 = load i32, i32* %V0, align 4
    store i32 %V2, i32* %V0
    %V3 = load i32, i32* %V0, align 4
    ret i32 %V3
}
