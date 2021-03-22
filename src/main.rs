use aeth::setting;
use plotters::prelude::*;

fn main() {
    let mut sm = setting::Streetmap::new(300.0, 300.0);
    sm.populate_blue_noise(300);

    let root = BitMapBackend::new("image-out/maps.png", (1024, 768)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let areas = root.split_by_breakpoints([944], [80]);

    let mut scatter_ctx = ChartBuilder::on(&areas[2])
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..400f64, 0f64..400f64)
        .unwrap();
    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()
        .unwrap();
    scatter_ctx
        .draw_series(sm.blocks.iter().map(|setting::BlockData { x, y, .. }| {
            Circle::new((*x as f64, *y as f64), 2, GREEN.filled())
        }))
        .unwrap();
}
