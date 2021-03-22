## qoeur compiler programming: language design

### productivity

to reduce the amounts of bug at compile time. the compiler implements `cli`, `assertions`, `type system`, `linters`, `package manager`, `auto-formatter`, `error handling`

### extensions

filename extensions: `.q5`

### assertions

unit testing will be embedded in the standard library. by this way testing can be standardized. using the `test` keyword instead of an attribute to applied assertions is required. also the language came with the `mock` and `bench`

`test` for tesing   
`mock` for mocking    
`bench` for benchmarking    

### imports

importing a file should be simple and will avoid side effect when importing another module

### package manager

embedding the package manager

### interpreter

the interpreter will be online, implemented via the `qoeur-lab` which is playground to discover the language. the `qoeur` interpreter must be keep tracking variables and functions in a simple way

### compiler

the compiler should blazing the compilation time to generate an executable for x86-64 architecture. first it will used `llvm` for the backend to accomplished his goal then new backend with no binary or compiler dependencies to generate machine code and linking by itself

### gui

embedded gui in the standard library to build beautiful interface for the web or native window application

```
<script>
  mut name: str = "";

  fun click = () {
    name = "world";
  }
</script>

<h1>hello, {name}!</h1>
<button @click={click}>try</button>
```

### webassembly

`wasm` for the web assembly feature

```
wasm = {
  # do something 
}
```
