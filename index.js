import('./pkg')
  .then(rust_module => {
    [
      [rust_module.Note.C3, "c"],
      [rust_module.Note.D3, "d"],
      [rust_module.Note.E3, "e"],
      [rust_module.Note.F3, "f"],
      [rust_module.Note.G3, "g"],
      [rust_module.Note.A3, "a"],
      [rust_module.Note.B3, "b"],
    ].forEach(([note, id]) => {
      const button = document.getElementById(id);
      button.addEventListener("click", async () => {
        const async_player = new rust_module.Player(note);
        player = await async_player;
        player.start();
      });
    });
  })
  .catch(console.error);
