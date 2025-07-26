<script lang="ts">
  import "bootstrap/dist/css/bootstrap.min.css";
  import { onMount } from "svelte";
  import { mode, map } from "./";
  import { MapLibre } from "svelte-maplibre";
  import {
    Layout,
    sidebarContents,
    mapContents,
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

  let sidebarDiv: HTMLDivElement | undefined;
  let mapDiv: HTMLDivElement | undefined;
  $effect(() => {
    if (sidebarDiv && $sidebarContents) {
      sidebarDiv.innerHTML = "";
      sidebarDiv.appendChild($sidebarContents);
    }
  });
  $effect(() => {
    if (mapDiv && $mapContents) {
      mapDiv.innerHTML = "";
      mapDiv.appendChild($mapContents);
    }
  });
</script>

<Layout>
  {#snippet left()}
    <h1>TEMPLATE TITLE</h1>

    <div bind:this={sidebarDiv}></div>
  {/snippet}

  {#snippet main()}
    <div style="position:relative; width: 100%; height: 100vh;">
      <MapLibre
        style={basemapStyles[basemap]}
        bind:map
        hash
        onerror={(e) => {
          console.log(e.error);
        }}
      >
        <StandardControls {map} />
        <MapContextMenu {map} />
        <Basemaps bind:basemap />

        <div bind:this={mapDiv}></div>

        {#if mode.kind == "title"}
          <TitleMode {wasmReady} />
        {:else if mode.kind == "main"}
          <MainMode />
        {/if}
      </MapLibre>
    </div>
  {/snippet}
</Layout>
