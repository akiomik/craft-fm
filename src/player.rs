use wasm_bindgen::prelude::*;

use crate::songs::Song;

#[wasm_bindgen]
pub struct Player {
    song: Option<Song>,
    is_playing: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Player {
        Self {
            song: None,
            is_playing: false,
        }
    }

    pub fn set_song(&mut self, song: Song) -> Result<(), JsValue> {
        self.stop()?;
        self.song = Some(song);
        Ok(())
    }

    pub fn play(&mut self) -> Result<(), JsValue> {
        if let Some(ref mut song) = self.song {
            song.stop()?;
            song.play()?;
            self.is_playing = true;
        }

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), JsValue> {
        self.is_playing = false;

        if let Some(ref mut song) = self.song {
            song.stop()?;
        }

        Ok(())
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }
}

#[cfg(test)]
mod tests {
    use crate::songs::Playable;

    use super::*;

    struct TestSong {}

    impl TestSong {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl From<TestSong> for Song {
        fn from(value: TestSong) -> Self {
            Song::new("test".into(), Box::new(value))
        }
    }

    impl Playable for TestSong {
        fn play(&mut self) -> Result<(), JsValue> {
            Ok(())
        }

        fn stop(&mut self) -> Result<(), JsValue> {
            Ok(())
        }
    }

    #[test]
    fn test_play_none() {
        let mut player = Player::new();
        assert!(!player.is_playing());
        player.play().unwrap();
        assert!(!player.is_playing());
    }

    #[test]
    fn test_play_some() {
        let mut player = Player::new();
        let song = TestSong::new().into();
        player.set_song(song).unwrap();
        assert!(!player.is_playing());
        player.play().unwrap();
        assert!(player.is_playing());
    }

    #[test]
    fn test_stop_none() {
        let mut player = Player::new();
        assert!(!player.is_playing());
        player.stop().unwrap();
        assert!(!player.is_playing());
    }

    #[test]
    fn test_stop_some() {
        let mut player = Player::new();
        let song = TestSong::new().into();
        player.set_song(song).unwrap();
        assert!(!player.is_playing());
        player.stop().unwrap();
        assert!(!player.is_playing());
    }

    #[test]
    fn test_play_and_stop_none() {
        let mut player = Player::new();
        assert!(!player.is_playing());
        player.play().unwrap();
        assert!(!player.is_playing());
        player.stop().unwrap();
        assert!(!player.is_playing());
    }

    #[test]
    fn test_play_and_stop_some() {
        let mut player = Player::new();
        let song = TestSong::new().into();
        player.set_song(song).unwrap();
        assert!(!player.is_playing());
        player.play().unwrap();
        assert!(player.is_playing());
        player.stop().unwrap();
        assert!(!player.is_playing());
    }
}
