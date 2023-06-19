pub trait PlayQueue {
    fn enqueue(&mut self, youtube_url: String);
    fn skip (&mut self);
}

pub struct PlayQueueFile {}

impl PlayQueue for PlayQueueFile {
    fn enqueue(&mut self, youtube_url: String) {
        println!("todo");
    }

    fn skip (&mut self) {
        todo!();
    }
}
