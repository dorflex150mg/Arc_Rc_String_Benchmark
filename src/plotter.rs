pub mod plotter {
    use plotters::prelude::*;
    
    const FILENAME: &str = "latency_hist.png";
    
    pub fn plot_hist(strings: Vec<u32>, rcs: Vec<u32>, arcs: Vec<u32>) -> 
        Result<(), Box<dyn std::error::Error>> {
    
        let root = BitMapBackend::new(FILENAME, (640, 480)).into_drawing_area();
    
        root.fill(&WHITE)?;
    
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption("Clone Latency Histogram by Data Structure", ("sans-serif", 50.0))
            .build_cartesian_2d((0u32..10u32).into_segmented(), 0u32..10u32)?;
    
        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(WHITE.mix(0.3))
            .y_desc("Count")
            .x_desc("Latency")
            .axis_desc_style(("sans-serif", 15))
            .draw()?;
    
        chart.draw_series(
            Histogram::vertical(&chart)
                .style(RED.mix(0.5))
                .data(strings.iter().map(|x: &u32| (*x, 1))),
        )?;
    
    
        root.present().expect("You are at the mercy of an angry god.");
        Ok(())
    }
}