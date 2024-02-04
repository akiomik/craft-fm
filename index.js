import('./pkg')
  .then((rust_module) => {
    const ctx = new AudioContext();
    const player = new rust_module.Player();

    const forest_button = document.getElementById('forest');
    forest_button.addEventListener('click', async () => {
      const promise = new rust_module.Forest(ctx, BigInt('42'));
      const forest = await promise;
      player.set_song(forest.into_song());
      player.play();
    });

    const metronome_button = document.getElementById('metronome');
    metronome_button.addEventListener('click', async () => {
      const promise = new rust_module.Metronome(ctx, 120);
      const metronome = await promise;
      player.set_song(metronome.into_song());
      player.play();
    });

    const toy808_button = document.getElementById('toy808');
    toy808_button.addEventListener('click', () => {
      const toy808 = new rust_module.Toy808(ctx, 140);
      player.set_song(toy808.into_song());
      player.play();
    });

    const stop_button = document.getElementById('stop');
    stop_button.addEventListener('click', () => {
      player.stop();
    });
  })
  .catch(console.error);
