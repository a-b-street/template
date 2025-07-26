<script lang="ts">
  import { GeoJSON, hoverStateFilter, LineLayer } from "svelte-maplibre";
  import { SplitComponent } from "svelte-utils/two_column_layout";
  import { mode, model } from "./";

  function backToTitle() {
    model.set(undefined);
    mode.set({ kind: "title" });
  }
</script>

<SplitComponent>
  {#snippet sidebar()}
    <button class="btn btn-danger" on:click={backToTitle}>
      Choose another area
    </button>
  {/snippet}

  {#snippet map()}
    <GeoJSON data={JSON.parse(model.getEdges())} generateId>
      <LineLayer
        paint={{
          "line-width": hoverStateFilter(5, 7),
          "line-color": "red",
        }}
      />
    </GeoJSON>
  {/snippet}
</SplitComponent>
