import('./pkg')
  .then(rust_module => {
    let player = null;

    const play_button = document.getElementById("play");
    play_button.addEventListener("click", async () => {
      if (player === null) {
        const async_player = new rust_module.Player();
        player = await async_player;
      }

      player.start();
    });
  })
  .catch(console.error);
