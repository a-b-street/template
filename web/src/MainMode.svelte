<script lang="ts">
  import { GeoJSON, hoverStateFilter, LineLayer } from "svelte-maplibre";
  import { SplitComponent } from "svelte-utils/two_column_layout";
  import { mode, model } from "./index.svelte.js";

  function backToTitle() {
    model.value = undefined;
    mode.value = { kind: "title" };
  }
</script>

<SplitComponent>
  {#snippet left()}
    <button class="btn btn-danger" on:click={backToTitle}>
      Choose another area
    </button>
  {/snippet}

  {#snippet main()}
    <GeoJSON data={JSON.parse(model.value!.getEdges())} generateId>
      <LineLayer
        manageHoverState
        paint={{
          "line-width": hoverStateFilter(5, 7),
          "line-color": "red",
        }}
      />
    </GeoJSON>
  {/snippet}
</SplitComponent>
