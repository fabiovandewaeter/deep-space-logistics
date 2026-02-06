<!-- tauri-app/src/App.svelte -->
<script lang="ts">
  import Counter from "./lib/Counter.svelte";

  let menuOpen = false;

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
