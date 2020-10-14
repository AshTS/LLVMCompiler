# To Do

## Language
* Add Preprocessor Support
    * Parsing
    * #include
    * #define, #undef
    * #macro
    * #ifdef, #ifndef, #endif #else
    * #error
* Add Inline Assembly Support
    * Add externing system
* Design a stdlib

## Intermediate Representation

* More Optimizations
    * Constant Folding
    * Add Register Reuse \[Done\]
    * Find domain of register \[Done\]
        * Find all paths from instruction \[Done\]

* Add better backups of registers between function calls
* Backup and Restore instructions

## Targets

### avrasm

* Finalize avrasm codegen
    * Compare implementation for 2 byte values
    * Arithmatic Operations
        * Addition
            * Add special cases for inc
        * Subtraction
            * Add special cases for dec
        * Multiplication
        * Division
        * AND
        * OR
        * XOR
    * Function Calls
    * Stack Usage

* Port stdlib to avrasm

### llvm

* Start llvm codegen