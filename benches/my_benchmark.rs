use criterion::{Criterion, criterion_group, criterion_main};
use regex::Regex;
use std::hint::black_box;
use std::path::PathBuf;
use std::sync::mpsc;

use simple_files_backup::modules::traverse::traverse_sources;
use simple_files_backup::modules::types::BackupCommand;

pub fn benchmark(c: &mut Criterion) {
  let (tx, _rx) = mpsc::channel::<BackupCommand>();
  let source_paths = vec![
    PathBuf::from("/home/q/.bashrc"),
    PathBuf::from("/home/q/.gitconfig"),
    PathBuf::from("/home/q/.Xmodmap"),
    PathBuf::from("/home/q/bin"),
    PathBuf::from("/home/q/.config"),
    PathBuf::from("/home/q/.ssh"),
  ];
  let target_path = PathBuf::from("/data/backup_2/");
  let ignore_pattern =
    Some(Regex::new(r"/(node_modules|.yarn|.next|target|yarn.lock)").unwrap());

  let mut group = c.benchmark_group("traverse_files");
  group.bench_function("traverse_files", |bencher| {
    bencher.iter(|| {
      black_box(traverse_sources(
        tx.clone(),
        source_paths.clone(),
        target_path.clone(),
        ignore_pattern.as_ref(),
      ))
    });
  });
  group.finish();
}

criterion_group! {
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = benchmark
}
criterion_main!(benches);
