use std::path::PathBuf;
use std::sync::mpsc;

use criterion::{Criterion, criterion_group, criterion_main};

use simple_files_backup::modules::types::BackupCommand;
use simple_files_backup::modules::utils::dirs::traverse_files;

pub fn benchmark(c: &mut Criterion) {
  let (tx, _rx) = mpsc::channel::<BackupCommand>();
  let source_paths = vec![
    PathBuf::from("/home/q/.bashrc"),
    PathBuf::from("/home/q/.gitconfig"),
    PathBuf::from("/home/q/.Xmodmap"),
    PathBuf::from("/home/q/bin"),
    PathBuf::from("/home/q/.config"),
    PathBuf::from("/home/q/.ssh"),
    // PathBuf::from("/home/q/Documents"),
  ];
  let target_path = PathBuf::from("/data/backup_2/");
  let mut group = c.benchmark_group("traverse_files");

  group.bench_function("traverse_files", |bencher| {
    bencher.iter(|| {
      traverse_files(
        tx.clone(),
        source_paths.clone(),
        target_path.clone(),
        None,
      )
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
