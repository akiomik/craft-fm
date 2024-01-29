import('./pkg')
  .then(async (rust_module) => {
    const ctx = new AudioContext();
    const player = new rust_module.Player();
    const metronome = await (new rust_module.Metronome(ctx, 120));
    player.set_song(metronome.into_song());

    const play_button = document.getElementById("play");
    play_button.addEventListener("click", () => {
      player.play();
    });

    const stop_button = document.getElementById("stop");
    stop_button.addEventListener("click", () => {
      player.stop();
    });
  })
  .catch(console.error);
