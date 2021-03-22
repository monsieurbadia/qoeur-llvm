# qoeur compiler programming: roadmap

### compiler

**architecture**

* [ ] x86-64
* [ ] wasm
* [ ] arm

**core**

* [x] syntax from saturn
* [x] tokenizer
* [ ] typechecker
* [ ] interpreter
* [ ] bytecoder
* [ ] backend | *`llvm`, `cranelift`, `assembly`, `web assembly`*
* [ ] reporter | *error messages a la `elm`*
* [ ] comment | *block, doc, line*
* [ ] primitives | *`int`, `uint`, `real`, `char`, `bool`, `str`, `(->)`,*
* [ ] variable | *bindings, immutable, lifetime tracking*
* [ ] function | *calls, first-class, higher-order*
* [ ] branches | *`if`, `else if`, `else`*
* [ ] pattern matching | *`match`*
* [ ] operator | *unary, binary*
* [ ] `loops` | *`for`, `for range`, `while`*
* [ ] `nil` | *not sure yet*
* [ ] `return` values
* [ ] `load` | *imports modules*
* [ ] closures
* [ ] multiple assignment | `val x y z : int = 0;`
* [ ] type system
* [ ] syscall
* [ ] data-structures | *`array`, `hashmap`, `struct`, `tuple`*
* [ ] detect pure functions
* [ ] literal | *hexadecimal, octal, binary*
* [ ] multi-threading
* [ ] unit testing included | `test`, `mock`, `bench` keywords
* [ ] assertions | *for input validation `expect!`, `must!` and `should!`*
* [ ] rewrite compiler in `qoeur`
* [ ] no garbage collector
* [ ] small binaries size
* [ ] fast compilation time
* [ ] ..

**linter**

* [ ] unused variables
* [ ] unused parameters
* [ ] unused `load`
* [ ] unmodified mutable variables
* [ ] ineffective assignments
* [ ] ..

**operators**

* [ ] `+`, `-`, `*`, `/`, `=`
* [ ] `+=`, `-=`, `*=`, `/=`, `==`, `!=`
* [ ] `<`, `<<`, `<=`, `>=`, `>>`, `>`
* [ ] `&`, `|`, `&&`, `||`
* [ ] `%`
* [ ] ..

**optimizations**

* [ ] exclude unused functions
* [ ] function call inlining
* [ ] expression optimization
* [ ] ..

### extensions

* [ ] vscode extension
* [ ] `IDE`
* [ ] ..

### tools

* [ ] repl | `--interactive`
* [ ] qoeur-lab
* [ ] package manager
