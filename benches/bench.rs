// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/maidsafe/QA/master/Images/maidsafe_logo.png",
    html_favicon_url = "https://maidsafe.net/img/favicon.ico",
    html_root_url = "https://docs.rs/parsec"
)]
#![forbid(
    arithmetic_overflow,
    mutable_transmutes,
    no_mangle_const_items,
    unknown_crate_types,
    warnings
)]
#![deny(
    bad_style,
    deprecated,
    improper_ctypes,
    non_shorthand_field_patterns,
    overflowing_literals,
    stable_features,
    unconditional_recursion,
    unknown_lints,
    unsafe_code,
    unused_allocation,
    unused_attributes,
    unused_comparisons,
    unused_features,
    unused_parens,
    while_true
)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
#![allow(
    box_pointers,
    missing_copy_implementations,
    missing_debug_implementations,
    variant_size_differences
)]

#[cfg(feature = "testing")]
#[macro_use]
extern crate criterion;

#[cfg(feature = "testing")]
#[macro_use]
extern crate unwrap;

#[cfg(feature = "testing")]
use criterion::Criterion;
#[cfg(feature = "testing")]
use parsec::dev_utils::Record;

#[cfg(feature = "testing")]
fn bench(c: &mut Criterion) {
    for name in &["minimal", "static", "dynamic"] {
        bench_dot_file(c, "benches", name);
    }

    for name in &[
        "a_node4_opaque_evt8",
        "a_node8_opaque_evt8",
        "a_node16_opaque_evt8",
        "a_node32_opaque_evt8",
        "a_node48_opaque_evt8",
    ] {
        bench_dot_file(c, "bench_section_size_evt8", name);
        bench_dot_file(c, "bench_section_size_evt8_single", name);
    }

    for name in &[
        "a_node4_opaque_evt16",
        "a_node8_opaque_evt16",
        "a_node16_opaque_evt16",
        "a_node32_opaque_evt16",
        "a_node48_opaque_evt16",
    ] {
        bench_dot_file(c, "bench_section_size_evt16", name);
        bench_dot_file(c, "bench_section_size_evt16_single", name);
    }

    for name in &[
        "a_node4_opaque_evt1024",
        "a_node8_opaque_evt1024",
        "a_node16_opaque_evt1024",
        "a_node32_opaque_evt1024",
    ] {
        bench_dot_file(c, "bench_section_size_evt1024_interleave", name);
        bench_dot_file(
            c,
            "bench_section_size_evt1024_interleave_supermajority",
            name,
        );
    }

    for name in &[
        "a_node4_opaque_evt8192",
        "a_node8_opaque_evt8192",
        "a_node16_opaque_evt8192",
        "a_node32_opaque_evt8192",
    ] {
        bench_dot_file(c, "bench_section_size_evt8192_interleave", name);
    }

    for name in &[
        "a_node4_opaque_evt65536",
        "a_node8_opaque_evt65536",
        "a_node16_opaque_evt65536",
        "a_node32_opaque_evt65536",
    ] {
        bench_dot_file(c, "bench_section_size_evt65536_interleave", name);
    }

    for name in &[
        "PublicIdname754598-001",
        "PublicIdname754598-002",
        "PublicIdname754598-003",
        "PublicIdname93b63e-001",
        "PublicIdname93b63e-002",
        "PublicIdname93b63e-003",
        "PublicIdname93b63e-004",
        "PublicIdname93b63e-005",
    ] {
        // To generate these benchmarks:
        //
        // 1. Edit benches/bench.rs (this file) and temporarily comment out the assert
        //    at the end of `bench_dot_file`.
        //
        // 2. Run the benchmarks with dump-graphs to generate the new dot files:
        //
        //     PARSEC_DUMP_GRAPH_MODE=on_parsec_drop PARSEC_DUMP_GRAPH_SVG=0 cargo bench --features=testing,dump-graphs -- --test PublicIdname
        //
        // 3. Copy the generated dot files:
        //
        //     cd [parsec dir]
        //     cp /tmp/parsec_graphs/latest/* input_graphs/bench_routing/mock_crust_merge_merge_three_sections_into_one/
        //
        // 4. Revert the changes from step 1.
        //
        bench_dot_file(
            c,
            "bench_routing/mock_crust_merge_merge_three_sections_into_one",
            name,
        );
    }
}

#[cfg(feature = "testing")]
fn bench_dot_file(c: &mut Criterion, group_name: &'static str, name: &'static str) {
    let test_name = format!("{} - {}", name, group_name);
    let _ = c.bench_function(&test_name, move |b| {
        let record = unwrap!(Record::parse(format!(
            "input_graphs/{}/{}.dot",
            group_name, name
        )));
        b.iter_with_setup(
            || record.clone(),
            |record| {
                let expected_history = record.consensus_history();

                let parsec = record.play();
                let actual_history = parsec.meta_election_consensus_history_hash();

                // Verify parsec reached the same consensus as in source dot file.
                // The last consensused element is not in dot file as it is added after generating the file.
                assert_eq!(expected_history, actual_history);
            },
        )
    });
}

#[cfg(feature = "testing")]
criterion_group! {
    name = benches;
    // Use smallest sample size to allow bigger/more numerous benchmarks
    config = Criterion::default().sample_size(3).noise_threshold(0.1);
    targets = bench
}

#[cfg(feature = "testing")]
criterion_main!(benches);

#[cfg(not(feature = "testing"))]
fn main() {
    println!("Benchmarks require `--features=testing`")
}
