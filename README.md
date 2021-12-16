# Penguin-Lang

This is a compiler for a language that provide a python-like interface to draw figures.

- use `cargo run`to compile and use it.
- when program launch, you can see an interface like

```shell
  >>>
```

If you enjoy writing code in file, it would be better for you to write the code in a file that has a suffix `.pg`, and
use is as input:

```shell
  $> penguin-lang file.pg
```

### Grammar supported:

1. origin is (\<num>, \<num>);
2. rot is \<num>;
3. scale is (\<num>, \<num>);
4. set x (\<num>, \<num>);
5. set y (\<num>, \<num>);
6. set color \<color\>;
7. set size n;
8. for t from \<num> to \<num> step \<num> draw(\<expr of t>, \<expr of t>);
9. show;
10. exit/quit/q

### Default values:
1. rot = 0
2. origin = (0, 0)
3. scale = (1, 1)
4. color = yellow
5. x = (0, 10)
6. y = (-4, 4)
7. size = 2
8. output -> /graph (**unchangeable now**)

## color support:
- blue
- red
- green
- black
- yellow
- cyan
- magenta

## math function support:
- cos
- sin
- tan
- ln
- exp
- sqrt

### Example
A proper example can be found in root directory: `code.pg`.

**Enjoy it! :)**


