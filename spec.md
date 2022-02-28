# Spec

## Comment

```
// ok
/* not ok */
```

## Line Break

### Semicolon is *NOT* optional 

```
let line1 = "line1"; // ok
let line2 = "line2"\n // not ok
```

## Variable

### A *Variable* always defined by assignment with `let` (no declaration),

```
let a = 1;
let a; // error
```

### And update freely (no `const` & *weak type*)

```
a = "2";
```

### A variable is a *Number* (all double),

```
let a = -3.14;
let a = 0xAF; // error
```

### Or a *String* (only double quote),

```
let a = "a";
let b = "abc";
let c = 'a'; // error
```

### Or a *Bool*,

```
let a = true; 
let b = false;
let c = False; // error
```

### Or root of all evil.

```
let a = null;
```

## Control Flow

To be turing-complete, we just have to include:

### *If*, and

```
if (true) {
    // braces MUST
}
```

### *For*

```
for (let i = 0; i < 10; i = i + 1) {
    // old school for impl simplicity
}
```

## Function

### A *Function* defined like:

```
function name(args) {
    return args;
}
```

### No `return` actually return `null`

```
function no_return() {
    // return null automatically, you are welcome
}
```

### And call MUST match args

```
name(42);
```

## Collection

### Once you got a *Table*, you got a *Object* and a *Array*

```
let a = [];
a[1] = 1;
a["2"] = "2";
a["3"] = {};
print(a["3"]);
```

Yes, we all love lua.

## Builtins (std library)

1. `print(1)`