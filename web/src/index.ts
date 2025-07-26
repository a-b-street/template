import type { Map } from "maplibre-gl";
import * as backend from "../../backend/pkg";

type Mode = { kind: "title" } | { kind: "main" };

// TODO The extra nesting is awful

export let model: { value: backend.Model | undefined } = $state({ value: undefined });
export let map: { value: Map | undefined } = $state({ value: undefined });
export let mode: { value: Mode } = $state({ value: { kind: "title" } });
