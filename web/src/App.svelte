<script lang="ts">
  import "bootstrap/dist/css/bootstrap.min.css";
  import { tick, onMount, untrack } from "svelte";
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
  $effect(() => {
    if (map.value) {
      tick().then(() => {
        map.value?.resize();
      });
    }
  });

  // svelte-ignore state_referenced_locally
  let initialStyle = basemapStyles.get(basemap)!;
  // TODO Upstream to svelte-maplibre. It feels a bit brittle how we detect the sources and layers created here, vs from the basemap
  $effect(() => {
    if (basemap) {
      untrack(() => {
        map.value?.setStyle(basemapStyles.get(basemap)!, {
          transformStyle: (previousStyle, nextStyle) => {
            if (!previousStyle) {
              return nextStyle;
            }

            let customLayers = previousStyle.layers.filter((l) =>
              ["circle-", "line-", "fill-", "heatmap-", "symbol-"].some(
                (prefix) => l.id.startsWith(prefix),
              ),
            );
            let layers = nextStyle.layers.concat(customLayers);
            let sources = nextStyle.sources;

            for (let [key, value] of Object.entries(
              previousStyle.sources || {},
            )) {
              if (key.startsWith("geojson-") || key.startsWith("heatmap")) {
                sources[key] = value;
              }
            }

            return {
              ...nextStyle,
              sources: sources,
              layers: layers,
            };
          },
        });
      });
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
        style={initialStyle}
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
