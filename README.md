# grep-like-tool

The problem statement is given [here](./project.pdf)

## Input Format

```
<number-of-test-cases>
<reg-ex-1>
<test-string-1>
.
.
.
```

Regular Expression Input follows the below grammar

```
R -> concat(R, R)
R -> union(R, R)
R -> star(R)
R -> symbol(C)
C -> <any-ascii-character>
```

For example, if `(a + b)^*c` is the regular expression, the input format will be

`concat(star(symbol(a)),union(symbol(b),symbol(c)))`

To test it,
specify the input in a test file, say `input.txt`

and run it with

```sh
cargo run < input.txt
```

If the string is part of the regular expression, output will be "Yes"
Else the output will be "No"

It has been tested with the sample input file given [here](./input.txt)
