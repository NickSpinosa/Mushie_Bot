use std::collections::{HashMap, VecDeque};

pub trait DownloadQueue {
    fn enqueue(&mut self, youtube_url: String);
}

pub struct DownloadQueueFile {
    song_map: HashMap<String, String>,
    queue: VecDeque<String>,
}

impl DownloadQueue for DownloadQueueFile {
    fn enqueue(&mut self, youtube_url: String) {
        self.queue.push_back(youtube_url);
    }
}
