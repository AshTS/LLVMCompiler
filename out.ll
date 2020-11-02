target triple = "avr-none"
define i32 @test(i64 %a)
{
    %V0 = alloca i64, align 8
    %V1 = load i64, i64* %V0, align 8
    ret i64 %V1
}
