# Rust Async Bench
This project is a simple demonstration of the overhead of Rust's `async`/`await`.

## Bench code
```rust
fn noop() -> i32 {
    black_box(0)
}

async fn noop_async() -> i32 {
    black_box(0)
}


fn sum(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

async fn sum_async(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

```
## Test Result
TL;DR `async/await` adds about 1.5 ns overhead per call
```
noop                    time:   [634.18 ps 640.81 ps 648.44 ps]                  
                        change: [-0.7535% -0.0095% +0.7528%] (p = 0.98 > 0.05)
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

noop_async              time:   [2.0463 ns 2.0598 ns 2.0747 ns]                        
                        change: [-1.2865% -0.8967% -0.5181%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe

sum_0                   time:   [383.43 ps 385.09 ps 386.81 ps]                  
                        change: [+0.4551% +0.8530% +1.2581%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

sum_0_async             time:   [2.0766 ns 2.0840 ns 2.0917 ns]                         
                        change: [-0.5450% -0.2032% +0.2024%] (p = 0.30 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

sum_1                   time:   [518.73 ps 520.59 ps 522.58 ps]                   
                        change: [-1.0106% -0.6360% -0.2347%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 16 outliers among 100 measurements (16.00%)
  5 (5.00%) low mild
  6 (6.00%) high mild
  5 (5.00%) high severe

sum_1_async             time:   [2.0912 ns 2.0970 ns 2.1033 ns]                         
                        change: [+0.8414% +1.2031% +1.5830%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

sum_2                   time:   [862.65 ps 866.46 ps 870.86 ps]                   
                        change: [-0.6294% -0.2090% +0.2294%] (p = 0.35 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

sum_2_async             time:   [2.2375 ns 2.2475 ns 2.2582 ns]                         
                        change: [+2.1973% +2.8836% +3.6437%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

sum_4                   time:   [1.9289 ns 1.9346 ns 1.9405 ns]                   
                        change: [+0.3805% +0.7433% +1.0777%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

sum_4_async             time:   [2.6000 ns 2.6069 ns 2.6146 ns]                         
                        change: [+0.1759% +0.7253% +1.2452%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

sum_10                  time:   [4.9823 ns 5.0017 ns 5.0247 ns]                    
                        change: [+1.1236% +1.6327% +2.1741%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild

sum_10_async            time:   [6.1486 ns 6.1774 ns 6.2068 ns]                          
                        change: [+0.9876% +1.3667% +1.7864%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

sum_20                  time:   [11.377 ns 11.422 ns 11.474 ns]                    
                        change: [+0.6980% +1.1450% +1.5801%] (p = 0.00 < 0.05)
                        Change within noise threshold.

sum_20_async            time:   [12.857 ns 12.918 ns 12.984 ns]                          
                        change: [+0.7883% +1.2265% +1.6891%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

sum_40                  time:   [24.442 ns 24.503 ns 24.565 ns]                    
                        change: [-0.3606% +0.0021% +0.3806%] (p = 0.99 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

sum_40_async            time:   [26.437 ns 26.537 ns 26.652 ns]                          
                        change: [-2.1719% -1.0847% -0.0691%] (p = 0.04 < 0.05)
                        Change within noise threshold.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low mild
  8 (8.00%) high mild
  4 (4.00%) high severe

sum_100                 time:   [67.114 ns 67.734 ns 68.418 ns]                    
                        change: [+2.4202% +3.1141% +3.8927%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

sum_100_async           time:   [70.875 ns 71.780 ns 72.907 ns]                          
                        change: [+5.7328% +6.8963% +8.3482%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

sum_200                 time:   [141.49 ns 142.31 ns 143.14 ns]                    
                        change: [+0.5756% +0.9764% +1.4155%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

sum_200_async           time:   [147.52 ns 148.95 ns 150.56 ns]                          
                        change: [+0.8133% +1.4739% +2.2214%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) high mild
  13 (13.00%) high severe

sum_400                 time:   [272.87 ns 273.60 ns 274.45 ns]                    
                        change: [+1.3213% +1.6511% +2.0297%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe

sum_400_async           time:   [276.13 ns 277.06 ns 278.15 ns]                          
                        change: [+1.4834% +1.9448% +2.4757%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 19 outliers among 100 measurements (19.00%)
  7 (7.00%) low mild
  5 (5.00%) high mild
  7 (7.00%) high severe

sum_1000                time:   [680.40 ns 684.55 ns 689.92 ns]                      
                        change: [-0.9163% -0.4377% +0.0697%] (p = 0.09 > 0.05)
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

sum_1000_async          time:   [665.36 ns 666.77 ns 668.20 ns]                            
                        change: [-0.5691% -0.1208% +0.2494%] (p = 0.59 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
```