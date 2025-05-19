use charming::{component::Axis, element::AxisType, series::Bar, Chart, WasmRenderer};
//use duckdb::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PageHit {
    path: String,
    value: i32,
}

pub fn getparqdata() -> Vec<PageHit> {
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
    data
}

pub fn chart() -> Chart {
    let data = getparqdata();
    let mut yy: Vec<String> = vec![];
    let mut xx: Vec<i32> = vec![];
    for d in data {
        yy.push(d.path);
        xx.push(d.value);
    }
    Chart::new()
        .y_axis(Axis::new().type_(AxisType::Category).data(yy))
        .x_axis(Axis::new().type_(AxisType::Value))
        .series(Bar::new().data(xx))
}

fn main() {
    let chart = chart();
    let renderer = WasmRenderer::new(1000, 800);
    // Render the chart in the WebAssembly runtime
    renderer.render("my-chart-id", &chart).unwrap();
}
