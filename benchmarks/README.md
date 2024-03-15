# Web frameworks benchmarking


## Specs

```
OS: Arch Linux x86_64 
Kernel: 6.6.16-1-lts 
CPU: 13th Gen Intel i5-13500H (16) @ 4.7GHz
GPU: Intel Raptor Lake-P [UHD Graphics] 
Memory: 7569MiB
Storage: 512GB SSD
```

## Results

run this command
```terminal
$ ./benchmark.sh
```

**actix-web**

```
1 thread and 1 connection
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    42.96us   77.37us   9.98ms   99.39%
    Req/Sec    24.04k     2.13k   26.33k    85.86%
  1437568 requests in 1.00m, 340.00MB read
Requests/sec:  23919.59
Transfer/sec:      5.66MB
```

***warp***

```
1 thread and 1 connection
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    36.35us   22.11us   3.11ms   97.87%
    Req/Sec    27.96k     2.48k   30.94k    78.87%
  1671659 requests in 1.00m, 396.96MB read
Requests/sec:  27814.82
Transfer/sec:      6.61MB
```

***axum***

```
1 thread and 1 connection
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    46.45us   54.40us   7.81ms   99.00%
    Req/Sec    21.99k     1.57k   23.88k    83.86%
  1314792 requests in 1.00m, 310.96MB read
Requests/sec:  21876.88
Transfer/sec:      5.17MB
```