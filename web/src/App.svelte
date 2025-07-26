<script lang="ts">
  import "bootstrap/dist/css/bootstrap.min.css";
  import { tick, onMount } from "svelte";
  import { mode, map } from "./index.svelte.js";
  import { MapLibre } from "svelte-maplibre";
  import {
    Layout,
    mainTarget,
    leftTarget,
  } from "svelte-utils/two_column_layout";
  import {
    basemapStyles,
    Basemaps,
    StandardControls,
    MapContextMenu,
  } from "svelte-utils/map";
  import TitleMode from "./TitleMode.svelte";
  import MainMode from "./MainMode.svelte";
  import * as backend from "../../backend/pkg";

  let wasmReady = $state(false);
  let basemap = $state("Maptiler Dataviz");

  onMount(async () => {
    await backend.default();
    wasmReady = true;
  });

  // TODO Hack
  $effect(async () => {
    if (map.value) {
      await tick();
      map.value?.resize();
    }
  });
</script>

<Layout>
  {#snippet left()}
    <h1>TEMPLATE TITLE</h1>

    <div bind:this={leftTarget.value}></div>
  {/snippet}

  {#snippet main()}
    <div style="position:relative; width: 100%; height: 100vh;">
      <MapLibre
        style={basemapStyles[basemap]}
        bind:map={map.value}
        hash
        onerror={(e) => {
          console.log(e.error);
        }}
      >
        <StandardControls map={map.value} />
        <MapContextMenu map={map.value} />
        <Basemaps bind:basemap />

        <div bind:this={mainTarget.value}></div>

        {#if mode.value.kind == "title"}
          <TitleMode {wasmReady} />
        {:else if mode.value.kind == "main"}
          <MainMode />
        {/if}
      </MapLibre>
    </div>
  {/snippet}
</Layout>
