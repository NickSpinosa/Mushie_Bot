use std::collections::HashMap;

use super::{download_queue::DownloadQueue, play_queue::PlayQueue};

pub struct Context<'a> {
    song_map: HashMap<String, String>,
    download_queue: &'a dyn DownloadQueue,
    play_queue: &'a dyn PlayQueue,
}
