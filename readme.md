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

## Lines
Given two lines 
(x11, y11) (x12, y12)
(x21, y21) (x22, y22)

## Potential Max
Max pressure given where we are is:
* Based on each position, find closest to each open valve

## Moves

```text
|....@..|
|....@..|
|..@@@..|
|.......|
|.......|
|.......|
|...#...|
|..###..|
|...#...|
|..####.|
+-------+

> 
|.....@.|
|.....@.|
|...@@@.|
|.......|
|.......|
|...#...|
|..###..|
|...#...|
|..####.|
+-------+

< 
|....@..|
|....@..|
|..@@@..|
|.......|
|...#...|
|..###..|
|...#...|
|..####.|
+-------+

< 
|...@...|
|...@...|
|.@@@...|
|...#...|
|..###..|
|...#...|
|..####.|
+-------+

< 
|.......|
|..#....|
|..#....|
|####...|
|..###..|
|...#...|
|..####.|
+-------+
|....#..|
|...###.|
|....#..|
|..#....|
|..#....|
|####...|
|..###..|
|...#...|
|..####.|
+-------+

|.......|
|....#..|
|...###.|
|..#.#..|
|..#....|
|####...|
|..###..|
|...#...|
|..####.|
+-------+

|.......|
|...#...|
|..###..|
|..##...|
|..#....|
|####...|
|..###..|
|...#...|
|..####.|
+-------+
```

Each time to decide if it can fit in we have to do the following:
* Check if we can move down
  * Check each overlapping row to see that the `&` is zero i.e. no bits are overlapping
  * If every overlapping row gives `0` then this is compatible at this position, then it can go down
* Now check if it can move at this level
  * Move the rock in the direction

Shapes: 5
Jets: 40
Width: 


0, 1, 2, 3, 4, 5, 6, 7, 8, 9
2, repeat of 3
10 - 2 = 8

# Spatial arrangement

After the initial scan we will get ranges like:

(x1, y, z) to (x2, y, z)
(x, y1, z) to (x, y2, z)
(x, y, z1) to (x, y, z2)
