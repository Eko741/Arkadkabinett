use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rsa::{pkcs8::DecodePrivateKey, RsaPrivateKey};
extern crate arkadkabinett;

fn criterion_benchmark(c: &mut Criterion) {
    const PRIV_KEY: &str = include_str!("private_key");
    let encrypted_message: String = include_str!("encrypted_message").to_string();
    let private_key = RsaPrivateKey::from_pkcs8_pem(PRIV_KEY).unwrap();

    c.bench_function("decrypting", |b| {
        b.iter(|| {
            arkadkabinett::security::decrypt_base64(
                black_box(encrypted_message.clone()),
                &private_key,
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
