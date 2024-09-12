# Append-only-benchmark

A quick evaluation of append-only lists

- https://lib.rs/crates/append-only-vec
- https://github.com/danieldulaney/appendlist
- https://docs.rs/appendbuf/
- https://docs.rs/elsa/latest/elsa/vec/struct.FrozenVec.html
- https://docs.rs/chunky-vec/
- https://docs.rs/segvec/

```
AppendOnlyVec           time:   [22.162 ms 22.337 ms 22.558 ms]
                        change: [-99.561% -99.552% -99.543%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

AppendList              time:   [20.193 ms 20.291 ms 20.394 ms]
                        change: [-91.243% -91.195% -91.143%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

FrozenVec               time:   [22.497 ms 22.580 ms 22.669 ms]
                        change: [-69.257% -69.074% -68.892%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

ChunkyVec               time:   [24.635 ms 24.768 ms 24.906 ms]
                        change: [-93.346% -93.308% -93.268%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

SegVec                  time:   [22.207 ms 22.294 ms 22.386 ms]
                        change: [-89.787% -89.737% -89.691%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

Vec                     time:   [22.454 ms 22.605 ms 22.759 ms]
                        change: [-67.922% -67.687% -67.447%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

SmallVec                time:   [22.843 ms 22.924 ms 23.007 ms]
                        change: [-90.227% -90.185% -90.143%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe

concurrent_list         time:   [22.181 ms 22.298 ms 22.440 ms]
                        change: [-94.122% -94.089% -94.054%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

pizza_arena             time:   [22.069 ms 22.150 ms 22.237 ms]
                        change: [-94.733% -94.712% -94.689%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

```