# scaf
Generate text from templates.

## Usage
`scaf` so far has only been tested on UTF-8 files. The template file syntax will also conflict with `sh` variables if there are conflicting names.

Here's an example template file, let's call it `foo.txt`:
```
The quick ${color} ${animal0} jumps over the lazy ${animal1}.
```
`color`, `animal0`, and `animal1` are variables that will be replaced if `scaf` is passed an argument that assigns one of those names, like below:

```sh
scaf --var color=green
     --var animal0=frog
     --var animal1=turtle
           foo.txt
```

This will print to stdout:
```
The quick green frog jumps over the lazy turtle.
```
Variables can omitted safely, so the template variable will not be replaced. However you must provide at least one file or `scaf` fails early.

With that said, multiple files can be used, with their contents first concatenated then replaced as necessary:

```
foo.txt
${msg1}
```
and
```
bar.txt
${msg2}
```

with this invocation:
```sh
scaf --var "msg1=Hello world!"
     --var "msg2=Goodbye friends!"
           foo.txt bar.txt
```

results in:
```
foo.txt
Hello world!
bar.txt
Goodbye friends!
```
