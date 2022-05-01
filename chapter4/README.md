# Lox Scanner

## Testing the book version (Java)

```
make java_chapters
```

Run chapter 4... 

```
java -cp build/gen/chap04_scanning com.craftinginterpreters.lox.Lox
```

Given

```
// A sample lox program
fun addPair(a, b) {
  return a + b;
}
 
fun identity(a) {
  return a;
}
print identity(addPair)(1, 2); // Prints "3".
```

we expect

```
FUN fun null
IDENTIFIER addPair null
LEFT_PAREN ( null
IDENTIFIER a null
COMMA , null
IDENTIFIER b null
RIGHT_PAREN ) null
LEFT_BRACE { null
RETURN return null
IDENTIFIER a null
PLUS + null
IDENTIFIER b null
SEMICOLON ; null
RIGHT_BRACE } null
FUN fun null
IDENTIFIER identity null
LEFT_PAREN ( null
IDENTIFIER a null
RIGHT_PAREN ) null
LEFT_BRACE { null
RETURN return null
IDENTIFIER a null
SEMICOLON ; null
RIGHT_BRACE } null
PRINT print null
IDENTIFIER identity null
LEFT_PAREN ( null
IDENTIFIER addPair null
RIGHT_PAREN ) null
LEFT_PAREN ( null
NUMBER 1 1.0
COMMA , null
NUMBER 2 2.0
RIGHT_PAREN ) null
SEMICOLON ; null
EOF  null
```
