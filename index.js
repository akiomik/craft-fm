import('./pkg')
  .then(async (rust_module) => {
    const promise = new rust_module.Player()
    const player = await promise;

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
      button.addEventListener("click", () => {
        player.play(note);
      });
    });
  })
  .catch(console.error);
