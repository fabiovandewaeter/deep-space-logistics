<!-- tauri-app/src/lib/Counter.svelte -->
<script>
  import { gameData, gameSnapshot } from "../wasm";
</script>

<main>
  <h2>Recipes</h2>
  <ul>
    {#each $gameData?.recipes.recipes ?? [] as t}
      <li>{t.name}</li>
    {/each}
  </ul>

  <h2>Terrain types</h2>
  <ul>
    {#each $gameData?.terrain_types.types ?? [] as t}
      <li>{t.name}</li>
    {/each}
  </ul>

  <h2>Transport types</h2>
  <ul>
    {#each $gameData?.transport_types.types ?? [] as t}
      <li>{t.name}</li>
    {/each}
  </ul>

  <h2>Snapshot</h2>
  {#if $gameSnapshot.length === 0}
    <p>Loading ...</p>
  {:else}
    <ul>
      {#each $gameSnapshot as item, i}
        <li>ID {i}: a={item.a} | b={item.b}</li>
      {/each}
    </ul>
  {/if}
</main>
