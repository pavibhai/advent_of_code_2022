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

# Geode Robots
Take for example
```text
Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian.
```

For ore production:
* 2 ores to create a new ore robot
* 3 mins before an ore robot is productive
* From this point onwards we double the production
* +3 mins
  * 2 ore robots, 1 ore
* +21 mins
  * 42 + 1 = 43

Each minute you can only build one robot:
* To get to a geode robot you need the following minimum minutes
  * 1 min for clay robot
  * 1 min for obsidian robot
  * 1 min for geode robot

What are the choices at each turn?
* Build a robot
  * Geode
  * Obsidian
  * Clay
  * Ore
  * Don't build any

let us say we always build the best robot if we can.

Fastest way to a geode collecting robot

# Encrypted File

```text
1, 2, -3, 3, -2, 0, 4
```

# Full Binary Tree
1
2
4
8

# Layout
```text
up
(0, -1)

left
(-1, 0)

down
(0, 1)

right
(1, 0)
```

Cube folding

R1|R2
In this transition

The cube layout can be like

```text
      R1
      --
R4|R3|R2
      --
      R6|R5
```

To identify adjacent regions do the following:
* If adjacent then it is adjacent
R1 d R2 d
R2 d R6 d, R2 u R1 u, R2 l R3 l
R3 r R2 r, R3 l R4 l
R4 r R3 r
R5 l R6 l
R6 r R5 r, R6 u R2 u

* Look at single connection of connection
  * If connecting down, ignore down connection, connect left and right
R1 d R2 d, R1 l R3 d,
R2 d R6 d, R2 u R1 u, R2 l R3 l, R2 r R5 d
R3 r R2 r, R3 l R4 l, R3 u R1 r, R3 d R6 r
R4 r R3 r
R5 l R6 l, R5 u R2 l
R6 r R5 r, R6 l R3 u, R6 u R2 u

* Look at double connection of connection
R1 d R2 d, R1 l R3 d, R1 r R5 l, R1 u R4 d
R2 d R6 d, R2 u R1 u, R2 l R3 l, R2 r R5 d
R3 r R2 r, R3 l R4 l, R3 u R1 r, R3 d R6 r
R4 r R3 r, R4 u R1 d, R4 d R6 u
R5 l R6 l, R5 u R2 l, R5 r R1 l
R6 r R5 r, R6 l R3 u, R6 u R2 u, R6 d R4 u

* Look at only remaining connection
R1 d R2 d, R1 l R3 d, R1 r R5 l, R1 u R4 d
R2 d R6 d, R2 u R1 u, R2 l R3 l, R2 r R5 d
R3 r R2 r, R3 l R4 l, R3 u R1 r, R3 d R6 r
R4 r R3 r, R4 u R1 d, R4 d R6 u, R4 l R5 u
R5 l R6 l, R5 u R2 l, R5 r R1 l, R5 d R4 r
R6 r R5 r, R6 l R3 u, R6 u R2 u, R6 d R4 u

```text
      R1
      --
R4|R3|R2
      --
      R6|R5
```


```text
R1
--
R2
```

* R1, down, R2
* R2, up, R1

```text
R3|R2
```
* R3, right, R2
* R2, left, R3

```text
   R1
   --
R3|**
```
* R1, left, R3 (down)
* R3, up, R1 (right)

```text
R2
--
**|R5
```
* R2, right, R5 (down)
* R5, up, R2 (left)