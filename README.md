# LLVMCompiler
A compiler from a C-like language to LLVM Intermediate representation.


## Syntax

The language this compiler works with is a C-like language with rust style type names.

### Types

The following is a list of all of the types in this language and their C equivalents.

```
i8      char
u8      unsigned char
i16     short
u16     unsigned short
i32     int
u32     unsigned int
i64     long
u64     unsigned long
void    void
```

### Functions

Functions are declared in the same way as in C, however there is no need to provide both a declaration and a definition for functions as functions can be given in any order. However, there is no overloading for functions. The following is a main function in both C, and the pseudo C of this compiler.

#### C
```c
int main(int argc, char** argv)
{
    return 0;
}
```

#### Pseudo C
```
i32 main(i32 argc, i8** argv)
{
    return 0;
}
```

### Literals

To simplify the type system, only integer literals are allowed.

### Expressions

The following is a list of all of the operations allowed in Pseudo-C

```
a[b]        Array Indexing
a(b, c)     Function Calls
a++         Post Incrementing
a--         Post Decrementing
++a         Pre Incrementing
--a         Pre Decrementing
+a          Unary Plus
-a          Unary Minus
!a          Logical Not
~a          Bitwise Not
*           Dereference
&           Reference
a*b         Multiplication
a/b         Division
a%b         Modulus
a+b         Addition
a-b         Subtraction
a<<b        Shift Left
a>>b        Shift Right
a<b         Less Than
a<=b        Less Than or Equal To
a>b         Greater Than
a>=b        Greater Than or Equal To
a==b        Equal
a!=b        Not Equal
a&b         Bitwise And
a^b         Bitwise Xor
a|b         Bitwise Or
a&&b        Logical And
a||b        Logical Or
a?b:c       Ternary
a=b         Assignment
a+=b        Add Assignment
a-=b        Subtract Assignment
a*=b        Multiply Assignment
a/=b        Divide Assignment
a%=b        Modulus Assignment
a<<=b       Shift Left Assignment
a>>=b       Shift Right Assignment
a&=b        Bitwise And Assignment
a^=b        Bitwise Xor Assignment
a|=b        Bitwise Or Assignment
a as u8     Type Cast
a, b, c     Comma
```

### Control Flow

There are three forms of loops in Pseudo C, `loop` which starts an infinite loop, `while` which is a standard while loop, and `do while` which is a standard do while loop. In addition there is the standard `if` `else if` `else` statements aswell. However, note that there are no parenthases required around the conditions for those control flow structures which require conditions.

In addition from within a loop the `continue` and `break` statements can be used. Finally, within a function the `return` statement can be used to return a value.

### Variable Declarations

Variable declarations are the same as in C with the new type names, and a value must be assigned at the declaration.

### Examples

#### factorial.pc
```
u32 factorial(u32 value)
{
    return (value < 2) ? 1 : (value * factorial(value - 1));
}
```

#### fibb.pc
```
u32 fibb(u32 index)
{
    u32 a = 0, b = 1, c = 0;

    while value--
    {
        c = b + a;
        a = b;
        b = c;
    }

    return b;
}
```