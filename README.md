# Optimizing brute-force solvers

Initial state:

```txt
solve_free_radical      time:   [19.749 ms 19.870 ms 20.013 ms]
```

Switch `smallvec` for `arrayvec`.

```txt
solve_free_radical      time:   [18.941 ms 19.047 ms 19.155 ms]
```

Add goal distance heuristic.

```txt
solve_free_radical      time:   [14.703 ms 14.879 ms 15.076 ms]
solve_fractal           time:   [26.020 ms 26.173 ms 26.331 ms]
```

Only consider goal-colored actors.

```txt
solve_free_radical      time:   [12.640 ms 12.791 ms 12.953 ms]
solve_fractal           time:   [10.420 ms 10.486 ms 10.557 ms]
```

Don't calculate transitions on previously visited states.

```txt
solve_free_radical      time:   [8.3712 ms 8.4585 ms 8.5543 ms]
solve_fractal           time:   [7.1456 ms 7.1885 ms 7.2328 ms]
```

## TODO

Symmetric state reduction, multithread
