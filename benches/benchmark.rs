use append_only_vec::AppendOnlyVec;
use appendbuf::AppendBuf;
use chunky_vec::ChunkyVec;
use criterion::{ criterion_group, criterion_main, Criterion};
use elsa::FrozenVec;
use smallvec::{SmallVec, smallvec};

#[derive(Debug, Clone, Default)]
pub struct Doc{
    pub name: String,
    pub desc: String,
    pub address: String,
    pub post: String,
    pub contact: String,
    pub phone: String,
    pub number: u32,
}

const ITEM_SIZE: u32 =1_000_000;

fn create_large_vector_v1() -> AppendOnlyVec<Doc> {
    let v = AppendOnlyVec::new();
    // Populate the vector with a large number of elements
    for i in 0..ITEM_SIZE {
        v.push(Doc::default());
    }
    v
}

fn create_large_vector_v2()  {

    use appendlist::AppendList;

    let list: AppendList<Doc> = AppendList::new();

    // Populate the vector with a large number of elements
    for i in 0..ITEM_SIZE {
        list.push(Doc::default());
    }
}



// fn create_large_vector_v3()  {
//     let mut v = AppendBuf::new(ITEM_SIZE as usize);
//     // Populate the vector with a large number of elements
//     for i in 0..ITEM_SIZE {
//         v.fill(Doc::default().into());
//     }
// }


fn create_large_vector_v4() ->FrozenVec<Doc> {
    let w= elsa::vec::FrozenVec::new();
    // Populate the vector with a large number of elements
    for i in 0..ITEM_SIZE {
        w.push(Doc::default());
    }
    w
}

fn create_large_vector_v5()  {
    let mut w= ChunkyVec::default();
    // Populate the vector with a large number of elements
    for i in 0..ITEM_SIZE {
        w.push(Doc::default());
    }
}
fn create_large_vector_v6()  {
    let mut w: segvec::SegVec<Doc>= segvec::SegVec::new();
    // Populate the vector with a large number of elements
    for i in 0..ITEM_SIZE {
        w.push(Doc::default());
    }
}
fn create_large_vector_v7()  {
    let mut w= Vec::new();
    // Populate the vector with a large number of elements
    for i in 0..ITEM_SIZE {
        w.push(Doc::default());
    }
}
fn create_large_vector_v8()  {
    let mut w: SmallVec<[Doc; 4]>= smallvec![];;
    // Populate the vector with a large number of elements
    for i in 0..ITEM_SIZE {
        w.push(Doc::default());
    }
}
fn create_large_vector_v9()  {
    let (mut w,r)= concurrent_list::new();
    // Populate the vector with a large number of elements
    for i in 0..ITEM_SIZE {
        w.push(Doc::default());
    }
}

fn create_large_vector_v10()  {
    let mut arena= pizza_common::arena::Arena::new(4,100_0000_00,1024*1024*1024);
    // Populate the vector with a large number of elements
    for i in 0..ITEM_SIZE {
        let doc=Doc::default();
        arena.must_alloc(doc);
    }
}


fn bench_create_dataset_v1(c: &mut Criterion) {
    c.bench_function("AppendOnlyVec", |b| {
        b.iter(|| {
            create_large_vector_v1()
        });
    });
}
fn bench_create_dataset_v2(c: &mut Criterion) {
    c.bench_function("AppendList", |b| {
        b.iter(|| {
            create_large_vector_v2()
        });
    });
}
// fn bench_create_dataset_v3(c: &mut Criterion) {
//     c.bench_function("AppendBuf", |b| {
//         b.iter(|| {
//             create_large_vector_v3()
//         });
//     });
// }
fn bench_create_dataset_v4(c: &mut Criterion) {
    c.bench_function("FrozenVec", |b| {
        b.iter(|| {
            create_large_vector_v4()
        });
    });
}
fn bench_create_dataset_v5(c: &mut Criterion) {
    c.bench_function("ChunkyVec", |b| {
        b.iter(|| {
            create_large_vector_v5()
        });
    });
}
fn bench_create_dataset_v6(c: &mut Criterion) {
    c.bench_function("SegVec", |b| {
        b.iter(|| {
            create_large_vector_v6()
        });
    });
}
fn bench_create_dataset_v7(c: &mut Criterion) {
    c.bench_function("Vec", |b| {
        b.iter(|| {
            create_large_vector_v7()
        });
    });
}

fn bench_create_dataset_v8(c: &mut Criterion) {
    c.bench_function("SmallVec", |b| {
        b.iter(|| {
            create_large_vector_v8()
        });
    });
}

fn bench_create_dataset_v9(c: &mut Criterion) {
    c.bench_function("concurrent_list", |b| {
        b.iter(|| {
            create_large_vector_v9()
        });
    });
}

fn bench_create_dataset_v10(c: &mut Criterion) {
    c.bench_function("pizza_arena", |b| {
        b.iter(|| {
            create_large_vector_v10()
        });
    });
}




fn bench_create_snapshot_v1(c: &mut Criterion) {
    c.bench_function("Create snapshot of large vector V1", |b| {
        let v = create_large_vector_v4();

        b.iter(|| {


        });
    });
}

// fn bench_create_snapshot_v2(c: &mut Criterion) {
//     c.bench_function("Create snapshot of large vector V2", |b| {
//         let v = create_large_vector_v2();
//         b.iter(|| {
//             let snapshot: test1::ReadonlySnapshot<u32> = v.readonly_snapshot();
//             // Ensure that the snapshot is not optimized away
//             black_box(snapshot);
//         });
//     });
// }

// fn bench_create_snapshot_v3(c: &mut Criterion) {
//     c.bench_function("Create snapshot of large vector V3", |b| {
//         let v = create_large_vector_v3();
//         b.iter(|| {
//             let snapshot= v.readonly_snapshot();
//             // Ensure that the snapshot is not optimized away
//             black_box(snapshot);
//         });
//     });
// }

criterion_group!(benches,
    bench_create_dataset_v1,
    bench_create_dataset_v2,
    // bench_create_dataset_v3,
    bench_create_dataset_v4,
    bench_create_dataset_v5,
    bench_create_dataset_v6,
    bench_create_dataset_v7,
    bench_create_dataset_v8,
    bench_create_dataset_v9,
    bench_create_dataset_v10,
    // bench_create_snapshot_v1,
    // bench_create_snapshot_v2,
    // bench_create_snapshot_v3,
);
criterion_main!(benches);
