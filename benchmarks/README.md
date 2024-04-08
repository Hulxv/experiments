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
**Actix**
```
Running 1m test @ http://127.0.0.1:8080
  1 threads and 1 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    24.34us   70.93us   8.52ms   99.45%
    Req/Sec    44.30k     4.18k   48.66k    90.35%
  2649355 requests in 1.00m, 626.60MB read
Requests/sec:  44082.87
Transfer/sec:     10.43MB
```

**Warp**
```
Running 1m test @ http://127.0.0.1:8083
  1 threads and 1 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    23.96us   93.11us  11.38ms   99.70%
    Req/Sec    44.65k     4.34k   49.74k    84.17%
  2665135 requests in 1.00m, 632.88MB read
Requests/sec:  44418.66
Transfer/sec:     10.55MB

```
***Axum***
```
Running 1m test @ http://127.0.0.1:8082
  1 threads and 1 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    23.83us   62.50us   8.81ms   99.56%
    Req/Sec    43.33k     3.59k   48.27k    73.04%
  2591651 requests in 1.00m, 612.95MB read
Requests/sec:  43122.68
Transfer/sec:     10.20MB
```


## Diagram
```
xychart-beta
    title "Sales Revenue"
    x-axis ["Actix-web", "Warp", "Axum"] 
    y-axis "Revenue (in $)" 10000 --> 60000
    bar [44082.87, 44418.66,43122.68]
   

```