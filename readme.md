* [2022](#2022)
  * [Mod Arithmetic](#ModArithmetic)
    * [Addition](#Addition)
    * [Multiplication](#Multiplication)
    * [Division](#Division)

# 2022 <a id="2022"></a>

## Mod Arithmetic <a id="ModArithmetic"></a>

* x: variable
* d: divisor
* q: quotient
* r: remainder

```text
x % d = r
x = q*d + r
```

### Addition <a id="Addition"></a>

Consider `(x + a) % d` where we want to add `a` and perform the mod

* substituting `x` we have `((q*d + r) + a) % d` i.e. `((q*d) + r + a) % d
* `q*d` is always divisible by d so can be removed resulting in `(r + a) % d`

### Multiplication <a id="Multiplication"></a>

Consider `(x * a) % d` where we want to multiply `a` and perform the mod

* substituting `x` we have `((q*d + r) * a) % d` i.e `((q*d*a) + (r * a) % d`
* `q*d*a` is always divisible by d so can be removed resulting in `(r * a) % d`

### Division <a id="Division"></a>

Consider `(x / t) % d` where we want to divide `a` and perform the mod

* `x / 3` is written as ((x - x % 3) / t)

3 % 5 = 3
3 / 3 % 5 = 1
4 % 5 / 3 = 4 / 3
4 / 3 % 5 = 1

| Number | % 3 | % 5 | %5 - %3 | Number / 3 % 5 |
|--------|-----|-----|---------|----------------|
| 1      | 1   | 1   | 0       | 0              |
| 2      | 2   | 2   | 0       | 0              |
| 3      | 0   | 3   | 3       | 1              |
