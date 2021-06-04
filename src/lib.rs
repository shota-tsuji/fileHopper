use notify::DebouncedEvent::Create;
use notify::{watcher, RecursiveMode, Watcher};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    // 確認対象のディレクトリにファイルを作成するとCreateイベントと作成したファイル名が返ってくる
    #[test]
    fn when_create_then_create_event() {
        const TARGET_DIR: &'static str = "/tmp/test/notify/";

        let taret_path = PathBuf::from(TARGET_DIR);
        let expected_path = PathBuf::from(TARGET_DIR.to_string() + "file1");

        let _ = fs::create_dir_all(taret_path.as_path());
        let _ = fs::remove_file(expected_path.as_path());

        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

        watcher
            .watch(taret_path.as_path(), RecursiveMode::Recursive)
            .unwrap();

        let _ = File::create(expected_path.as_path());

        let result = rx.recv().unwrap();
        assert_eq!(Create(expected_path), result);
    }
}
