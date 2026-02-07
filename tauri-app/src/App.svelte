<!-- tauri-app/src/App.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import Counter from "./lib/Counter.svelte";
  import { initGame } from "./wasm";

  let menuOpen = true;

  onMount(async () => {
    try {
      console.log("Loading game ...");
      await initGame();
      console.log("Game started !");
    } catch (e) {
      console.error("Error while loading game:", e);
    }
  });

  function toggleMenu() {
    menuOpen = !menuOpen;
  }
</script>

<main>
  <button on:click={toggleMenu}>
    {menuOpen ? "Close menu" : "Open menu"}
  </button>

  {#if menuOpen}
    <div class="card">
      <Counter />
    </div>
  {/if}

  <h1>Vite + Svelte</h1>
</main>

<style>
</style>
