// This bench was used to decide to use RUST-TLS vs OPENSSL
// RUST-TLS was faster by a landslide
// RUST-TLS == 1.4ms
// OPENSSL == 64ms

use influxdb_rs::{Client};
use url::Url;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use tokio::runtime::Runtime;

async fn client_setup() -> Result<(), influxdb_rs::error::Error> {
    // Create client with a parsed url, bucket, org, and jwt token
    Client::new(Url::parse("http://localhost:8086").unwrap(), "test_bucket", "test_org", "0123456789").await?;

    Ok(())
}

fn tls_bench(c: &mut Criterion) {

    let mut group = c.benchmark_group("IO_BenchMarks");

    group.measurement_time(std::time::Duration::from_secs(10));


    group.bench_function("create_delete_database", |b | {

        b.to_async(Runtime::new().unwrap()).iter(|| client_setup());
    });
}

criterion_group!(benches, tls_bench);
criterion_main!(benches);