use criterion::{Criterion, criterion_group, criterion_main};
use inline_option::{IOption, Nullable};

#[derive(Clone, Default)]
#[repr(transparent)]
struct TestU32(u32);

impl Nullable for TestU32 {
    const NULL: Self = TestU32(0);

    fn is_null(&self) -> bool {
        self.0 == 0
    }
}

fn bench_iter_mut<T: Nullable + Clone + Default>(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_iter_mut");
    group.throughput(criterion::Throughput::Elements(10000));
    let mut vec = vec![None::<T>; 10000];
    group.bench_function("STD Option", |b| {
        b.iter(|| {
            for x in vec.iter_mut() {
                x.replace(T::default());
            }
        })
    });

    let mut vec = vec![IOption::<T>::none(); 10000];
    group.bench_function("Inline Option", |b| {
        b.iter(|| {
            for x in vec.iter_mut() {
                x.replace(T::default());
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_iter_mut::<TestU32>,);
criterion_main!(benches);
