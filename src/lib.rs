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

        let dir_location = PathBuf::from(TARGET_DIR);
        let file_location = PathBuf::from(TARGET_DIR.to_string() + "file1");

        let _ = fs::create_dir_all(dir_location.as_path());
        let _ = fs::remove_file(file_location.as_path());

        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

        watcher
            .watch(dir_location.as_path(), RecursiveMode::Recursive)
            .unwrap();

        let _ = File::create(file_location.as_path());

        assert_eq!(Create(file_location), rx.recv().unwrap());
    }
}
