* [2022](#2022)
  * [Mod Arithmetic](#ModArithmetic)
    * [Addition](#Addition)
    * [Multiplication](#Multiplication)

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

### Division

Consider `(x / t) % d` where we want to divide `a` and perform the mod

* `x / 3` is written as ((x - x % t) / t)

3 % 5 = 3
3 / 3 % 5 = 1
4 % 5 / 3 = 4 / 3
4 / 3 % 5 = 1

(1 / 3) % 5 = 0 % 5 = 0 = 1 % 5 / 3
(2 / 3) % 5 = 0 % 5 = 0 = 2 % 5 / 3
(3 / 3) % 5 = 1 % 5 = 1 = 3 % 5 / 3
(4 / 3) % 5 = 1 % 5 = 1 = 4 % 5 / 3
(5 / 3) % 5 = 1 % 5 = 1 = 5 % 5 / 3
(6 / 3) % 5 = 2 % 5 = 2
(7 / 3) % 5 = 2 % 5 = 2
(8 / 3) % 5 = 2 % 5 = 2
(9 / 3) % 5 = 3 % 5 = 3
(10 / 3) % 5 = 3 % 5 = 3
(11 / 3) % 5 = 3 % 5 = 3
(12 / 3) % 5 = 4 % 5 = 4
(13 / 3) % 5 = 4 % 5 = 4
