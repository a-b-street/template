<script lang="ts">
  import * as backend from "../../backend/pkg";
  import { PolygonToolLayer } from "maplibre-draw-polygon";
  import { OverpassSelector } from "svelte-utils/overpass";
  import type { Feature, Polygon } from "geojson";
  import { Loading } from "svelte-utils";
  import { onMount } from "svelte";
  import { SplitComponent } from "svelte-utils/two_column_layout";
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

  let fileInput: HTMLInputElement;
  async function loadFile(e: Event) {
    try {
      loading = "Loading from file";
      let bytes = await fileInput.files![0].arrayBuffer();
      gotModel(new backend.Model(new Uint8Array(bytes)));
    } catch (err) {
      window.alert(`Bad input file: ${err}`);
    } finally {
      loading = "";
    }
  }

  async function gotXml(xml: string, boundary: Feature<Polygon>) {
    try {
      let bytes = new TextEncoder().encode(xml);
      gotModel(new backend.Model(new Uint8Array(bytes)));
    } catch (err) {
      window.alert(`Couldn't import from Overpass: ${err}`);
    } finally {
      loading = "";
    }
  }

  function gotModel(m: backend.Model) {
    model.value = m;
    mode.value = { kind: "main" };
    // TODO zoom fit
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

    <div>
      <label class="form-label">
        Load an osm.pbf or osm.xml file
        <input
          class="form-control"
          bind:this={fileInput}
          onchange={loadFile}
          type="file"
        />
      </label>
    </div>

    <p class="fst-italic my-3">or...</p>

    <OverpassSelector
      map={map!.value}
      {gotXml}
      onloading={(msg) => (loading = msg)}
      onerror={(msg) => window.alert(msg)}
    />
  {/snippet}

  {#snippet main()}
    <PolygonToolLayer />
  {/snippet}
</SplitComponent>
