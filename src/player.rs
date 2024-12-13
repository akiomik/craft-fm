use std::{cell::RefCell, panic, rc::Rc};

use wasm_bindgen::prelude::*;

use crate::{result::Result, songs::Song, worker::WebWorker};

#[wasm_bindgen]
pub struct Player {
    song: Option<Rc<RefCell<Song>>>,
    worker: WebWorker,
    is_playing: bool,
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Player> {
        panic::set_hook(Box::new(console_error_panic_hook::hook)); // TODO

        let worker = WebWorker::new("./worker.js")?;

        Ok(Self {
            song: None,
            is_playing: false,
            worker,
        })
    }

    pub fn set_song(&mut self, song: Song) -> Result<()> {
        self.stop()?;
        self.song = Some(Rc::new(RefCell::new(song)));
        Ok(())
    }

    pub fn play(&mut self) -> Result<()> {
        if let Some(song_ref) = self.song.clone() {
            self.worker.set_onmessage(move |message| {
                if message.data() == "tick" {
                    song_ref.borrow_mut().tick().expect("should tick");
                }
            });

            self.worker.post_message("start")?;
            self.is_playing = true;
        }

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        self.worker.post_message("stop")?;
        self.is_playing = false;

        Ok(())
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }
}

#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;

    use super::*;
    use crate::songs::Playable;

    struct TestSong {}

    impl TestSong {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl From<TestSong> for Song {
        fn from(value: TestSong) -> Self {
            Song::new("test", Box::new(value))
        }
    }

    impl Playable for TestSong {
        fn tick(&mut self) -> Result<()> {
            Ok(())
        }
    }

    #[wasm_bindgen_test]
    pub fn test_play_none() {
        let mut player = Player::new().unwrap();
        assert!(!player.is_playing());
        player.play().unwrap();
        assert!(!player.is_playing());
    }

    #[wasm_bindgen_test]
    pub fn test_play_some() {
        let mut player = Player::new().unwrap();
        assert!(!player.is_playing());
        let song = TestSong::new().into();
        player.set_song(song).unwrap();
        assert!(!player.is_playing());
        player.play().unwrap();
        assert!(player.is_playing());
    }

    #[wasm_bindgen_test]
    pub fn test_stop_none() {
        let mut player = Player::new().unwrap();
        assert!(!player.is_playing());
        assert!(!player.is_playing());
        player.stop().unwrap();
        assert!(!player.is_playing());
    }

    #[wasm_bindgen_test]
    pub fn test_stop_some() {
        let mut player = Player::new().unwrap();
        let song = TestSong::new().into();
        player.set_song(song).unwrap();
        assert!(!player.is_playing());
        player.stop().unwrap();
        assert!(!player.is_playing());
    }

    #[wasm_bindgen_test]
    pub fn test_play_and_stop_none() {
        let mut player = Player::new().unwrap();
        assert!(!player.is_playing());
        player.play().unwrap();
        assert!(!player.is_playing());
        player.stop().unwrap();
        assert!(!player.is_playing());
    }

    #[wasm_bindgen_test]
    pub fn test_play_and_stop_some() {
        let mut player = Player::new().unwrap();
        let song = TestSong::new().into();
        player.set_song(song).unwrap();
        assert!(!player.is_playing());
        player.play().unwrap();
        assert!(player.is_playing());
        player.stop().unwrap();
        assert!(!player.is_playing());
    }
}
