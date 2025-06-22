pub mod dbnet;
pub mod det_arrange;
pub mod imageproc;
pub mod opencv;

// infer | macos | python
// Run 1: 3.3190 seconds
// Run 2: 3.4639 seconds
// Run 3: 3.3502 seconds
//

// detect | macos | bench
// Run 1-100: [2.2406 s 2.2678 s 2.2999 s]
// Found 5 outliers among 100 measurements (5.00%)
//   2 (2.00%) high mild
//   3 (3.00%) high severe
//
//
// So on macos it performs 35% better. Its not a perfect comparison, because the rust implementations is using coreml while python is using the cpu. The performance can probably be improved by setting the intra_threads and inter_threads, i chose (4, 2) without benchmarking what works best on my machine
