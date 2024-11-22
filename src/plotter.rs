pub mod plotter {
    use plotters::prelude::*;

    pub struct Dimensions {
        x: u32,
        y: u32,
    }

    impl Dimensions {
        pub fn new(x: u32, y: u32) -> Dimensions {
            Dimensions {
                x,
                y,
            }
        }
    }


    impl Default for Dimensions {
        fn default() -> Dimensions {
            Dimensions {
                x: 300,
                y: 1000,
            }
        }
    }
    
    
    pub fn plot_hist(filename: &str, strings: Vec<u32>, rcs: Vec<u32>, arcs: Vec<u32>, dimensions: Option<Dimensions>) -> 
        Result<(), Box<dyn std::error::Error>> {
    
        let dimension = match dimensions {
            Some(d) => d,
            None => Dimensions::default(),
        };

        let root = BitMapBackend::new(filename, (640, 480)).into_drawing_area();
    
        root.fill(&WHITE)?;
    
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(10)
            .caption("Clone Latency Histogram by Data Structure (R: String, B: Rc, G: Arc)", ("sans-serif", 10.0))
            .build_cartesian_2d((0u32..dimension.x).into_segmented(), 0u32..dimension.y)?;
    
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
                .style(GREEN.filled())
                .margin(5)
                .data(arcs.iter().map(|x: &u32| (*x, 1))),
        )?;

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(BLUE.filled())
                .margin(5)
                .data(rcs.iter().map(|x: &u32| (*x, 1))),
        )?;
    
        chart.draw_series(
            Histogram::vertical(&chart)
                .style(RED.filled())
                .margin(5)
                .data(strings.iter().map(|x: &u32| (*x, 1))),
        )?;
    
        root.present().expect("You are at the mercy of an angry god.");
        Ok(())
    }
}
