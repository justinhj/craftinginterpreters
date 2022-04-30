# Challenge

1. There are at least six domain-specific languages used in the little system I cobbled together to write and publish this book. What are they?

"The build scripts, test runner, and other utilities are all written in Dart."

WIP

2. Get a “Hello, world!” program written and running in Java. Set up whatever Makefiles or IDE projects you need to get it working. If you have a debugger, get comfortable with it and step through your program as it runs.

3. Do the same thing for C. To get some practice with pointers, define a doubly-linked list of heap-allocated strings. Write functions to insert, find, and delete items from it. Test them.

See `./c-double-linked-list`

## Build

Uses make to build, and since I'm using [clangd]() for lsp support I need to run this to make a json file for it to use when the project changes.

`make` and `./main`

```
cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=1
```

## Clangd install notes

From brew

==> llvm
To use the bundled libc++ please add the following LDFLAGS:
  LDFLAGS="-L/usr/local/opt/llvm/lib -Wl,-rpath,/usr/local/opt/llvm/lib"

llvm is keg-only, which means it was not symlinked into /usr/local,
because macOS already provides this software and installing another version in
parallel can cause all kinds of trouble.

If you need to have llvm first in your PATH, run:
  echo 'export PATH="/usr/local/opt/llvm/bin:$PATH"' >> ~/.zshrc

For compilers to find llvm you may need to set:
  export LDFLAGS="-L/usr/local/opt/llvm/lib"
  export CPPFLAGS="-I/usr/local/opt/llvm/include"

## Tests

Uses [CuTest](https://github.com/asimjalis/cutest)

`make` and `./tests`
