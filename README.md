# square-circle-compiler (October 2022)
This is the front-end of a compiler I made in Rust for a SPU course this past quarter that compiles programs written in a made-up programming language called "square-circle".
## BNF Grammar:
```
PROGRAM     -->   definitions: 
                     DEFS
                  operations:
                     OPERATIONS
                  end.
DEFS        -->   DEF | DEF; DEFS
DEF         -->   ID = point(NUM, NUM) |
                  ID = circle(ID, NUM) |
                  ID = square(ID, NUM)
OPERATIONS  -->   OPERATION | OPERATION; OPERATIONS
OPERATION   -->   print(ID) |
                  contained(ID, ID) |
                  intersects(ID, ID)
ID          -->   LETTER+
NUM         -->   DIGIT+
LETTER      -->   a | b | c | d | e | f | g | ... | z
NUM         -->   0 | 1 | 2 | 3 | 4 | 5 | 6 | ... | 9
```

The tokens (some lexemes are examples):
| Token | Lexeme |
| ----- | ------ |
| `ID` | `alpha` |
| `NUM` |  `256` |
| `SEMICOLON` | `;` |
| `COLON` | `:` |
| `COMMA` | `,` |
| `PERIOD` | `.` |
| `LPAREN` | `(` |
| `RPAREN` | `)` |
| `ASSIGN` | `=` |
| `DEFINITIONS` | `definitions` |
| `OPERATIONS` | `operations` |
| `POINT` | `point` |
| `CIRCLE` | `circle` |
| `SQUARE` | `square` |
| `PRINT` | `print` |
| `CONTAINED` | `contained` |
| `INTERSECTS` | `intersects` |
| `END` | `end` |
