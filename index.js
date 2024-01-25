import('./pkg')
  .then(async (rust_module) => {
    const promise = new rust_module.Player();
    const player = await promise;

    [
      [rust_module.Note.C3, "c3"],
      [rust_module.Note.Csharp3, "c#3"],
      [rust_module.Note.D3, "d3"],
      [rust_module.Note.Dsharp3, "d#3"],
      [rust_module.Note.E3, "e3"],
      [rust_module.Note.F3, "f3"],
      [rust_module.Note.Fsharp3, "f#3"],
      [rust_module.Note.G3, "g3"],
      [rust_module.Note.Gsharp3, "g#3"],
      [rust_module.Note.A3, "a3"],
      [rust_module.Note.Asharp3, "a#3"],
      [rust_module.Note.B3, "b3"],
      [rust_module.Note.C4, "c4"],
      [rust_module.Note.Csharp4, "c#4"],
      [rust_module.Note.D4, "d4"],
    ].forEach(([note, id]) => {
      const button = document.getElementById(id);
      button.addEventListener("click", () => {
        player.play(note);
      });
    });

    const stop_button = document.getElementById("stop");
    stop_button.addEventListener("click", () => {
      player.stop();
    });
  })
  .catch(console.error);
