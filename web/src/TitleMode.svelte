<script lang="ts">
  import * as backend from "../../backend/pkg";
  import { PolygonToolLayer } from "maplibre-draw-polygon";
  import { OsmLoader } from "abstreet-svelte-utils/osm";
  import type { Feature, Polygon } from "geojson";
  import { Loading } from "abstreet-svelte-utils";
  import { bbox } from "abstreet-svelte-utils/map";
  import { onMount } from "svelte";
  import { SplitComponent } from "abstreet-svelte-utils/two_column_layout";
  import { map, mode, model } from "./index.svelte.js";

  let { wasmReady }: { wasmReady: boolean } = $props();

  let loading = $state("");

  let examples: string[] = $state([]);
  let loadExample = $state("");

  onMount(async () => {
    try {
      let resp = await fetch("example_osm/list");
      if (resp.ok) {
        examples = await resp.json();
      }
    } catch (err) {}
  });

  async function loadFromExample() {
    if (loadExample.length == 0) {
      return;
    }
    try {
      loading = "Loading from example file";
      let resp = await fetch(`example_osm/${loadExample}`);
      let bytes = await resp.arrayBuffer();
      gotModel(new backend.Model(new Uint8Array(bytes)));
    } catch (err) {
      window.alert(`Bad input file: ${err}`);
    } finally {
      loading = "";
    }
  }

  async function onload(
    osmInput: Uint8Array,
    boundary: Feature<Polygon> | null,
  ) {
    try {
      gotModel(new backend.Model(osmInput));
    } catch (err) {
      window.alert(`Couldn't import: ${err}`);
    } finally {
      loading = "";
    }
  }

  function gotModel(m: backend.Model) {
    model.value = m;
    mode.value = { kind: "main" };
    zoomToFit();
  }

  function zoomToFit() {
    if (map.value && model.value) {
      map.value.fitBounds(bbox(JSON.parse(model.value.getEdges())), {
        animate: false,
        padding: 10,
      });
    }
  }
</script>

<Loading {loading} />

<SplitComponent>
  {#snippet left()}
    {#if examples.length}
      <div>
        <label>
          Load an example
          <select
            class="form-select"
            bind:value={loadExample}
            onchange={loadFromExample}
          >
            {#each examples as x}
              <option value={x}>{x}</option>
            {/each}
          </select>
        </label>
      </div>

      <p class="fst-italic my-3">or...</p>
    {/if}

    <OsmLoader
      map={map!.value}
      {onload}
      onloading={(msg) => (loading = msg)}
      onerror={(msg) => window.alert(msg)}
    />
  {/snippet}

  {#snippet main()}
    <PolygonToolLayer />
  {/snippet}
</SplitComponent>
