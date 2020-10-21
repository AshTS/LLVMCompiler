target triple = "avr-none"
define void @main()
{
    store i8 7, i8* inttoptr (i64 36 to i8*)
    store i8 5, i8* inttoptr (i64 37 to i8*)
    br label %L0

  L0:
    br label %L0
    ret void
}
