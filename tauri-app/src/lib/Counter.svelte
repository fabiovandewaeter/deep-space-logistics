<!-- tauri-app/src/lib/Counter.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import { initWasm, stepOnce } from "./wasm";

  let snapshot: Array<{ a: number; b: string }> = [];

  onMount(async () => {
    await initWasm();

    // option A: requestAnimationFrame loop
    function tick() {
      const s = stepOnce(); // stepOnce retourne promise si async wrapper
      Promise.resolve(s).then((v) => {
        snapshot = v;
        requestAnimationFrame(tick);
      });
    }
    requestAnimationFrame(tick);

    // option B: setInterval every N ms
    // setInterval(async () => {
    //   snapshot = await stepOnce();
    // }, 16);
  });
</script>

<main>
  <h2>Snapshot</h2>
  <ul>
    {#each snapshot as item, i}
      <li>{i}: a={item.a} b={item.b}</li>
    {/each}
  </ul>
</main>
