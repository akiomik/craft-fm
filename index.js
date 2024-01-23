import('./pkg')
  .then(rust_module => {
    const c_button = document.getElementById("c");
    c_button.addEventListener("click", async () => {
      const async_player = new rust_module.Player(rust_module.Note.C3);
      player = await async_player;
      player.start();
    });

    const d_button = document.getElementById("d");
    d_button.addEventListener("click", async () => {
      const async_player = new rust_module.Player(rust_module.Note.D3);
      player = await async_player;
      player.start();
    });

    const e_button = document.getElementById("e");
    e_button.addEventListener("click", async () => {
      const async_player = new rust_module.Player(rust_module.Note.E3);
      player = await async_player;
      player.start();
    });

    const f_button = document.getElementById("f");
    f_button.addEventListener("click", async () => {
      const async_player = new rust_module.Player(rust_module.Note.F3);
      player = await async_player;
      player.start();
    });

    const g_button = document.getElementById("g");
    g_button.addEventListener("click", async () => {
      const async_player = new rust_module.Player(rust_module.Note.G3);
      player = await async_player;
      player.start();
    });

    const a_button = document.getElementById("a");
    a_button.addEventListener("click", async () => {
      const async_player = new rust_module.Player(rust_module.Note.A3);
      player = await async_player;
      player.start();
    });

    const b_button = document.getElementById("b");
    b_button.addEventListener("click", async () => {
      const async_player = new rust_module.Player(rust_module.Note.B3);
      player = await async_player;
      player.start();
    });
  })
  .catch(console.error);
