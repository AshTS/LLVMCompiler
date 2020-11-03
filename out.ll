define void @strcpy(i8* %dest, i8* %src)
{
    %V0 = alloca i8*, align 8
    %V1 = alloca i8*, align 8
    store i8* %dest, i8** %V1
    %V2 = alloca i8*, align 1
    %V3 = alloca i8*, align 8
    %V4 = alloca i8*, align 8
    store i8* %src, i8** %V4
    %V5 = alloca i8, align 1
    br label %L0

  L0:
    %V6 = load i8*, i8** %V1, align 8
    store i8* %V6, i8** %V0
    %V8 = load i8*, i8** %V1, align 8
    %V7 = add i8* %V8, inttoptr
    store %V7, i8** %V1
    %V9 = load i8*, i8** %V0, align 8
    %V10 = load i8*, i8** %V2, align 1
    store i8* %V9, i8* %V10
    %V11 = load i8*, i8** %V4, align 8
    store i8* %V11, i8** %V3
    %V13 = load i8*, i8** %V4, align 8
    %V12 = add i8* %V13, inttoptr
    store %V12, i8** %V4
    %V15 = load i8*, i8** %V3, align 8
    %V14 = load i8, i8* %V15, align 1
    store i8 %V14, i8* %V5
    %V16 = load i8, i8* %V5, align 1
    %V17 = load i8*, i8** %V2, align 1
    store i8 %V16, i8* %V17
    %V19 = load i8, i8* %V5, align 1
    %V18 = icmp ne i8 %V19, 0
    br i1 %V18, label %L0, label %L2
    br label %L2

  L2:
    %V20 = alloca void, align 0
    %V21 = load void, void* %V20, align 0
    ret void
}
