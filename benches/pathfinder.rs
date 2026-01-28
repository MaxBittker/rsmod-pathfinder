use std::fs;

use criterion::*;
use criterion::measurement::WallTime;

use rsmod::rsmod::{CollisionStrategies, Normal, PathFinder};
use rsmod::rsmod::collision::collision::CollisionFlagMap;

fn bench_pathfinder(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("pathfinder");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut collision = CollisionFlagMap::new();

    let lumbridge: Vec<Vec<i32>> =
        serde_json::from_str(&fs::read_to_string("lumbridge.json").expect("")).expect("");

    // apply the flags to the mapsquare area.
    for x in 3200..3264 {
        for z in 3200..3264 {
            unsafe {
                collision.set(
                    x,
                    z,
                    0,
                    lumbridge[((z & 0x3f) | ((x & 0x3f) << 6) | ((0 & 0x3) << 12)) as usize]
                        [((x & 0x7) | ((z & 0x7) << 3)) as usize] as u32,
                );
            }
        }
    }

    let pathfinder: PathFinder = PathFinder::new();

    group.bench_function("find_path_short_128x128", move |b| {
        b.iter_batched(
            || pathfinder.clone(),
            |mut pathfinder| unsafe {
                pathfinder.find_path(
                    &collision,
                    0,
                    3232,
                    3205,
                    3232,
                    3205 + 10,
                    1,
                    1,
                    1,
                    0,
                    -1,
                    true,
                    0,
                    25,
                    &CollisionStrategies::Normal(Normal),
                );
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_long_pathfinder(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("long_pathfinder");
    group.throughput(Throughput::Elements(1));

    let mut collision = CollisionFlagMap::new();

    let lumbridge: Vec<Vec<i32>> =
        serde_json::from_str(&fs::read_to_string("lumbridge.json").expect("")).expect("");

    // apply the flags to the mapsquare area (64x64 tiles)
    for x in 3200..3264 {
        for z in 3200..3264 {
            unsafe {
                collision.set(
                    x,
                    z,
                    0,
                    lumbridge[((z & 0x3f) | ((x & 0x3f) << 6) | ((0 & 0x3) << 12)) as usize]
                        [((x & 0x7) | ((z & 0x7) << 3)) as usize] as u32,
                );
            }
        }
    }

    // 512x512 grid pathfinder for long-distance paths
    let long_pathfinder: PathFinder = PathFinder::with_size(512, 16384);

    // Benchmark: short path (10 tiles) with 512x512 grid
    let collision_clone = collision.clone();
    let pathfinder_clone = long_pathfinder.clone();
    group.bench_function("find_path_short_512x512", move |b| {
        b.iter_batched(
            || pathfinder_clone.clone(),
            |mut pathfinder| unsafe {
                pathfinder.find_path(
                    &collision_clone,
                    0,
                    3232,
                    3205,
                    3232,
                    3205 + 10,
                    1,
                    1,
                    1,
                    0,
                    -1,
                    true,
                    0,
                    25,
                    &CollisionStrategies::Normal(Normal),
                );
            },
            BatchSize::SmallInput,
        )
    });

    // Benchmark: medium path (50 tiles) with 512x512 grid
    let collision_clone = collision.clone();
    let pathfinder_clone = long_pathfinder.clone();
    group.bench_function("find_path_medium_512x512", move |b| {
        b.iter_batched(
            || pathfinder_clone.clone(),
            |mut pathfinder| unsafe {
                pathfinder.find_path(
                    &collision_clone,
                    0,
                    3232,
                    3220,
                    3232,
                    3220 + 40,
                    1,
                    1,
                    1,
                    0,
                    -1,
                    true,
                    0,
                    50,
                    &CollisionStrategies::Normal(Normal),
                );
            },
            BatchSize::SmallInput,
        )
    });

    // Benchmark: long path (corner to corner of mapsquare, ~90 tiles diagonal)
    let collision_clone = collision.clone();
    let pathfinder_clone = long_pathfinder.clone();
    group.bench_function("find_path_long_512x512", move |b| {
        b.iter_batched(
            || pathfinder_clone.clone(),
            |mut pathfinder| unsafe {
                pathfinder.find_path(
                    &collision_clone,
                    0,
                    3200,
                    3200,
                    3263,
                    3263,
                    1,
                    1,
                    1,
                    0,
                    -1,
                    true,
                    0,
                    100,
                    &CollisionStrategies::Normal(Normal),
                );
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_pathfinder, bench_long_pathfinder);

criterion_main!(benches);
