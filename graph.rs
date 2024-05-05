use plotters::prelude::*;
use std::collections::HashMap;

pub fn generate_bar_chart(
    data: &HashMap<(String, String), u32>,
    output_file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new(output_file_path, (640, 480)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let max_deaths = data.values().max().copied().unwrap_or(0) + 10;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("COVID-19 Deaths by State and Condition", ("sans-serif", 40))
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .build_cartesian_2d(
            0..data.len() as i32,
            0..max_deaths as i32,
        )?;

    chart.configure_mesh()
        .x_labels(15)
        .y_labels(10)
        .y_label_formatter(&|x| format!("{:}", x))
        .x_label_formatter(&|x| format!("{:}", x))
        .draw()?;

    let mut bars = Vec::new();
    for (idx, ((_, _), &deaths)) in data.iter().enumerate() {
        bars.push((
            idx as i32,
            0,
            idx as i32 + 1,
            deaths as i32,
        ));
    }

    for bar in bars {
        chart.draw_series(std::iter::once(Rectangle::new(
            [(bar.0, bar.1), (bar.2, bar.3)],
            BLUE.filled(),
        )))?;
    }

    root_area.present()?;
    Ok(())
}
