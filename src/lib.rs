use charming::{
    component::Axis,
    element::{AxisType, Color},
    series::Bar,
    Chart, WasmRenderer,
};
//use polars::prelude::*;
//use std::io::Cursor;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct PageHit {
    path: String,
    value: i32,
}

#[wasm_bindgen]
pub fn chart(_url: &str) {
    //let bb = getParqData(url);
    //let reader = Cursor::new(&bb);
    //let result = ParquetReader::new(reader).finish().unwrap();
    //let a = result.select(["foo"]);
    //log(&format!("{:?}", a));
    let mut data: Vec<PageHit> = vec![];
    data.push(PageHit {
        path: "Mon".to_string(),
        value: 150,
    });
    data.push(PageHit {
        path: "Tue".to_string(),
        value: 200,
    });
    data.push(PageHit {
        path: "Wed".to_string(),
        value: 150,
    });
    data.push(PageHit {
        path: "Thu".to_string(),
        value: 80,
    });
    data.push(PageHit {
        path: "Fri".to_string(),
        value: 70,
    });
    data.push(PageHit {
        path: "Sat".to_string(),
        value: 110,
    });
    data.push(PageHit {
        path: "Sun".to_string(),
        value: 130,
    });

    let mut yy: Vec<String> = vec![];
    let mut xx: Vec<i32> = vec![];
    for d in data {
        yy.push(d.path);
        xx.push(d.value);
    }
    let chart = Chart::new()
        .y_axis(Axis::new().type_(AxisType::Category).data(yy))
        .x_axis(Axis::new().type_(AxisType::Value))
        .series(
            Bar::new()
                .data(xx)
                .item_style(Color::Value("#337777".to_string())),
        );

    let renderer = WasmRenderer::new(1000, 800);
    // Render the chart in the WebAssembly runtime
    renderer.render("my-chart-id", &chart).unwrap();
}

#[wasm_bindgen]
extern "C" {
    pub fn getParqData(s: &str) -> Vec<u8>;

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
