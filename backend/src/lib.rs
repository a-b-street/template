#[allow(unused_imports)]
#[macro_use]
extern crate anyhow;
#[allow(unused_imports)]
#[macro_use]
extern crate log;

use std::sync::Once;

use anyhow::Result;
use geojson::GeoJson;
use utils::{Tags, osm2graph::Graph};
use wasm_bindgen::prelude::*;

static START: Once = Once::new();

#[wasm_bindgen]
pub struct Model {
    graph: Graph,
}

#[wasm_bindgen]
impl Model {
    #[wasm_bindgen(constructor)]
    pub fn new(input_bytes: &[u8]) -> Result<Model, JsValue> {
        // Panics shouldn't happen, but if they do, console.log them.
        console_error_panic_hook::set_once();
        START.call_once(|| {
            console_log::init_with_level(log::Level::Info).unwrap();
        });

        let mut graph = Graph::new(input_bytes, keep_edge, &mut utils::osm2graph::NullReader)
            .map_err(err_to_js)?;
        graph.compact_ids();

        Ok(Self { graph })
    }

    #[wasm_bindgen(js_name = getEdges)]
    pub fn get_edges(&self) -> Result<String, JsValue> {
        let mut features = Vec::new();
        for (id, edge) in &self.graph.edges {
            let mut f = self.graph.mercator.to_wgs84_gj(&edge.linestring);
            f.set_property("edge_id", id.0);
            features.push(f);
        }
        serde_json::to_string(&GeoJson::from(features)).map_err(err_to_js)
    }
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}

fn keep_edge(tags: &Tags) -> bool {
    if !tags.has("highway")
        || tags.is_any("highway", vec!["construction", "proposed"])
        || tags.is("area", "yes")
    {
        return false;
    }
    true
}
