## qoeur compiler programming: syntax

defining a relevant syntax that makes semantically meaningful is not easy. many programming language designers neglect these aspects completely. in general, a syntax should be simple, consistent, intuitive and flexible (a bit like legos).

my choices in terms of syntax focus on the perfectible points to be detected in certain languages. these choices only concern me and in no way represent the procedure to follow.

the syntax allows you to take pleasure in programming, it must also arouse a certain curiosity as well as a positive tension.

inspirations: [`elm`](https://en.wikipedia.org/wiki/Elm_(programming_language)), [`erlang`](https://en.wikipedia.org/wiki/Erlang_(programming_language)), [`fjölnir`](https://en.wikipedia.org/wiki/Fj%C3%B6lnir_(programming_language)), [`hermes`](https://en.wikipedia.org/wiki/Hermes_(programming_language)) [`icon`](https://en.wikipedia.org/wiki/Icon_(programming_language)), [`jai`](https://en.wikipedia.org/wiki/Draft:Jai_(programming_language)), [`planc`](https://en.wikipedia.org/wiki/PLANC), [`rust`](https://en.wikipedia.org/wiki/Rust_(programming_language))

font: [fira code](https://github.com/tonsky/FiraCode)

### comments

```
# this is a line comment

#+
this is a block comment
#-

#!+
#! this is a block doc comment 
#!-
```

### literals

```
false: bool
true: bool

42: int # `uint`, `s32`, `u32`, `s64`, `u64`

# note: `float` is the more appropriate word for  but the design of
# the `real` keyword looks nice so.. we will see later
1.0: real # `r32`, `r64` 

'a': char
"abc": str

"""
this is a multi-line string
"""
```

### entry

```
fun main = () {
  print!("hello, world! 👽");
}
```

### modules

```
load @my_module;

use @std::sys::(exit);
use @std::mem::(alloc, free);
```

### functions

types system (stil in research..)

```
# functions declaration
fun mul: (x: int, y: int) = (x, y) {
  x * y
}

# closures
fun sqrt: (\x -> int) = (x) -> x * x;

# call
sqrt(mul(1, 3));

# currying research
fun curry: (\ int * int -> \x -> \y -> x + y) = (f) {
  fun = (x) {
    fun = (y) {
      f(x, y)
    }
  }
}

# currying research
fun curry: (\int, int -> \x -> \y -> x + y) = (f) {
  fun = (x) {
    fun = (y) {
      f(x, y)
    }
  }
}

# currying research
fun curry: ((n: int) -> (int) -> int) = (n) {
  (x) -> n + x
}

# call curried
val curried: int = curry(mul);
print!("{}", curried(1)(2));
```



### bindings

**immutables**

```
# immutable variables via `val` keyword
val hello: str = "hello,";
val world: str = " world! 👽";
val greet: str = hello world +; # or maybe `hello ++ world` or `hello + world`
```

**mutables**

mutable variable via `mut` keyword

```
mut y: int = 1;
mut x: int = 1_000_000;
mut z: real = 3.0;
```

**multiple assignments**

```
val x y z : int = 0;
mut x y z : real = 1.0;
```

### branches

**if**

```
if a > b {
  a
} else {
  b
}
```

### structs

```
struct Button {
  name: str,
  id: int,
}

val button: Button = Button { id = 0, name = "button-name" };

set Button {
  fun new: Button = (name: str) {
    Self {
      id = 0,
      name = name,
    }
  }
}

val button: Button = new::Button("button-name");
```

### loops

**loop**

```
loop {
  print!("ouloulou!")
}
```

**for**

```
for elmts $elmt  {
  print!("{}", $elmt)
}

for elmts {
  print!("{}", $elmt)
}
```

**while**

```
val mut x y := 0;

while x < 3 {
  y = y + 2;
  x = x + 1;
}
```

### pattern matching

```
match ch {
  '+' => add(x, y),
  '-' => sub(x, y),
  '*' => mul(x, y),
  '/' => div(x, y),
  _ => @panic("with msg")
}
```

### ranges

```
# repeat 3 times
for 0..3 {
  print!("hello: {}", $it)
}

# repeat 3 times and assign loop counter to 'i'
for 0..3 = $it {
  print!("hello", $it)
}
```

### capsules

`capsule` representing interface for point struct

```
capsule Vec2 {
  fun mul: (-> int) = (.) -> .x * .x;
}

|> derive: clone, debug.
struct Point {
  x: real,
  y: real,
}

set Vec2 for Point {
  fun mul: (-> real) = (.) -> {
    .x * .y
  }
}
```


### enum

```
enum Vec2 {
  BasicEnum,
  Struct { .x: int },
}
```

### array

```
[10, 23, 12, 3]
```

### collections

```
.{
  firstname = "",
  lastname = "",
};
```

### goto

still in research..

```
\ok

# do something..

goto \ok
```

```
@ok

goto @ok
```

```
ok:

goto ok
```

### macros

the `$..` token for macro declaration

```
# alternative 1
!! token {
  # do something
}

# alternative 2
$.. token {
  # do something
}

# alternative 3
macro token {
  # do something
}
```

### annotations

**declaration type**

```
type YourType = real;
```

**inferred types**

the type is inferred by using `:=`

```
val a := true;
val b := false;
val c := a == b;
fun add := (x, y) { x + y }
```

**optional types**

```
fun do_something: (-> | foo: ?A) = (foo) {
  if foo {
    foo.bar = 0;
  }
}
```

### assertions

`|>` representing an attribute for the qoeur compiler

```
|> cfg: test.
suite {
  mock tokenization_mock = () {
    # mock computation
  }

  test tokenization_test = () {
    val x y := true;
    must!(x be y)
  }
}
```

### benchmark

```
bench tokenization_benchmark = () {
  # bench computation
}
```

### ffi

**c**

`ext` call for `c` function     

```
ext fun sqrt: (-> int | x: int);
```

**javascript**

`exp` call for `javascript` function    

```
exp fun cos: (-> int | x: int);
```

**rust**

`mod` call for `rust` function  

```
mod fun sqrt: (-> int | x: int);
```

### webassembly

```
wasm = {}
```
