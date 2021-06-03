use criterion::{black_box, criterion_group, criterion_main, Criterion};
use entropy::shannon_entropy;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = br#"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque molestie, odio
quis porta tincidunt, odio arcu scelerisque mauris, eu lobortis mi felis nec
nunc. Donec et hendrerit ligula. Phasellus odio tellus, eleifend et libero sit
amet, hendrerit bibendum augue. Nunc sit amet accumsan sapien, in porta risus.
Nulla est erat, viverra in suscipit sit amet, laoreet volutpat lacus. Fusce elit
enim, viverra ac felis in, sagittis pharetra nunc. Maecenas tempor arcu a
finibus congue. Nulla sed neque sodales, gravida magna at, convallis mi.

Morbi fermentum tortor vel elit eleifend egestas. Etiam facilisis suscipit
tortor, eu scelerisque nulla sollicitudin vel. Sed malesuada lorem sit amet
magna semper faucibus. Nulla tincidunt enim a ligula pellentesque pretium. Nulla
facilisis lacus eu sodales imperdiet. Vivamus et placerat enim. Maecenas
imperdiet viverra feugiat. Aliquam et varius enim, sed vulputate lectus.

Nam posuere auctor iaculis. Nam et nibh tempus, commodo lectus non, luctus
lectus. Aliquam erat volutpat. Aliquam erat volutpat. Maecenas sapien sapien,
molestie varius varius sed, pretium sit amet elit. Vivamus et eleifend diam.
Duis blandit metus nisi, quis gravida ante maximus eu. Donec faucibus, augue a
rhoncus aliquet, enim sapien blandit lacus, a vestibulum elit neque a lorem.

Etiam et magna placerat, scelerisque diam vitae, congue nisi. In varius porta
sem id dapibus. Sed venenatis velit id cursus lacinia. Etiam laoreet diam dolor,
eu malesuada urna placerat non. Praesent eu porttitor nibh. Nunc non congue
sapien, et sodales sapien. Sed luctus lacus a metus commodo, at porta arcu
gravida. Proin at sagittis mi. Nulla a enim at ipsum porta porta. Praesent ut
dolor sapien. Curabitur maximus ultrices leo at vulputate.

Cras ornare, lorem pellentesque commodo condimentum, diam risus commodo ante,
vel varius est massa ut tortor. Suspendisse venenatis mi justo, ac euismod risus
faucibus eget. Duis a ultricies ipsum. Nam a arcu tortor. Nulla viverra turpis
eget nisi ullamcorper dictum. Sed elementum, felis quis convallis vestibulum,
magna libero pulvinar ante, eget posuere magna justo blandit neque. Proin
aliquam hendrerit sodales."#;

    c.bench_function("shannon_long", |b| b.iter(|| shannon_entropy(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
