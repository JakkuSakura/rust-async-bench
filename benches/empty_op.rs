use criterion::{black_box, criterion_group, criterion_main, Criterion};

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

fn criterion_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
    c.bench_function("noop", |b| b.iter(|| black_box(noop())));
    c.bench_function("noop_async", |b| {
        b.to_async(&rt).iter(|| async { black_box(noop_async().await) })
    });
    for i in vec![0, 1, 2, 4, 10, 20, 40, 100, 200, 400, 1000] {
        c.bench_function(&format!("sum_{}", i), |b| b.iter(|| black_box(sum(i))));
        c.bench_function(&format!("sum_{}_async", i), |b| {
            b.to_async(&rt).iter(|| async { black_box(sum_async(i).await) })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);