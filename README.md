# Append-only-benchmark

A quick evaluation of append-only lists

- https://lib.rs/crates/append-only-vec
- https://github.com/danieldulaney/appendlist
- https://docs.rs/appendbuf/
- https://docs.rs/elsa/latest/elsa/vec/struct.FrozenVec.html
- https://docs.rs/chunky-vec/
- https://docs.rs/segvec/

## Push 100 items

```
AppendOnlyVec           time:   [679.63 ns 683.71 ns 688.60 ns]

AppendList              time:   [245.59 ns 246.30 ns 246.97 ns]

AppendBuf               time:   [330.80 ns 331.89 ns 333.24 ns]

FrozenVec               time:   [212.12 ns 212.79 ns 213.55 ns]

ChunkyVec               time:   [246.16 ns 248.15 ns 250.47 ns]

SegVec                  time:   [348.83 ns 350.00 ns 351.32 ns]

Vec                     time:   [210.54 ns 212.71 ns 215.90 ns]
```

## Push 1_000 items

```
AppendOnlyVec           time:   [6.1221 µs 6.1308 µs 6.1407 µs]

AppendList              time:   [2.1678 µs 2.1715 µs 2.1755 µs]

AppendBuf               time:   [3.2496 µs 3.2570 µs 3.2656 µs]

FrozenVec               time:   [756.11 ns 760.09 ns 764.46 ns]

ChunkyVec               time:   [2.1582 µs 2.1648 µs 2.1719 µs]

SegVec                  time:   [2.2448 µs 2.2528 µs 2.2613 µs]

Vec                     time:   [734.94 ns 736.51 ns 737.97 ns]=
```

## Push 10_000 items

```
AppendOnlyVec           time:   [68.015 µs 68.724 µs 69.444 µs]

AppendList              time:   [19.815 µs 19.868 µs 19.922 µs]

AppendBuf               time:   [31.907 µs 32.017 µs 32.139 µs]

FrozenVec               time:   [4.9989 µs 5.0569 µs 5.1235 µs]

ChunkyVec               time:   [22.235 µs 22.320 µs 22.422 µs]

SegVec                  time:   [21.147 µs 21.430 µs 21.714 µs]

Vec                     time:   [4.9329 µs 4.9673 µs 5.0012 µs]
```

## Push 1_00_000 items
```
AppendOnlyVec           time:   [2.5633 ms 2.6909 ms 2.8239 ms]

AppendList              time:   [199.17 µs 201.05 µs 203.93 µs]

AppendBuf               time:   [319.07 µs 319.68 µs 320.37 µs]

FrozenVec               time:   [51.599 µs 51.920 µs 52.218 µs]

ChunkyVec               time:   [245.03 µs 245.88 µs 246.76 µs]

SegVec                  time:   [201.63 µs 203.08 µs 204.42 µs]

Vec                     time:   [39.042 µs 39.268 µs 39.638 µs]
```


## Push 1_000_000 items
```
AppendOnlyVec           time:   [41.873 ms 43.746 ms 45.599 ms]

AppendList              time:   [2.1085 ms 2.1257 ms 2.1465 ms]

AppendBuf               time:   [3.2314 ms 3.2725 ms 3.3379 ms]

FrozenVec               time:   [563.90 µs 569.61 µs 575.87 µs]

ChunkyVec               time:   [2.5146 ms 2.5616 ms 2.6240 ms]

SegVec                  time:   [2.1164 ms 2.1408 ms 2.1745 ms]

Vec                     time:   [531.68 µs 535.08 µs 538.85 µs]

SmallVec                time:   [2.3164 ms 2.3191 ms 2.3217 ms]
```

## Push 10_000_000 items
```
AppendOnlyVec           time:   [502.33 ms 519.17 ms 535.83 ms]

AppendList              time:   [22.126 ms 22.183 ms 22.243 ms]

AppendBuf               time:   [31.935 ms 32.001 ms 32.070 ms]

FrozenVec               time:   [9.0269 ms 9.1197 ms 9.2275 ms]

ChunkyVec               time:   [27.346 ms 27.412 ms 27.485 ms]

SegVec                  time:   [21.204 ms 21.301 ms 21.443 ms]

Vec                     time:   [9.1011 ms 9.1654 ms 9.2295 ms]
```

## Push 100_000_000 items
```
AppendList              time:   [231.54 ms 232.97 ms 234.87 ms]

AppendBuf               time:   [323.79 ms 324.72 ms 325.85 ms]

FrozenVec               time:   [70.970 ms 71.251 ms 71.558 ms]

ChunkyVec               time:   [280.37 ms 282.26 ms 284.62 ms]

SegVec                  time:   [217.63 ms 218.05 ms 218.49 ms]

Vec                     time:   [67.259 ms 67.418 ms 67.589 ms]
```