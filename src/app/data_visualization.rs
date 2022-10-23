// Using the plot definitions within the whole scope since this is 
// the data visualization source code anyways.
use egui::plot::{Line, PlotPoint, PlotPoints};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct DataVisualization {

    // First test plot: populations
    iteration : Vec<u32>,
    bacters_population : Vec<u32>,
    algae_population : Vec<u32>,
}

// Public plotting functions
 
impl DataVisualization{

    pub fn default() -> Self{
        let temp = DataVisualization{
            iteration : Vec::<u32>::new(),
            bacters_population : Vec::<u32>::new(),
            algae_population : Vec::<u32>::new(),
        }

        iteration.push(30);
        iteration.push(60);
        iteration.push(100);
        bacters_population.push(300);
        bacters_population.push(350);
        bacters_population.push(340);
    }

    pub fn population_plot(& self, ui: &mut egui::Ui) -> egui::Response {
        let bacters_points = self.iteration.iter()
            .zip(self.bacters_population.iter())
            .map(|(i, v)| [*i as f64, *v as f64])
            .collect::<Vec<[f64; 2]>>()
            .to_owned();

        let bacters_line = Line::new(PlotPoints::from(bacters_points));
        //let algae_line = Line::new(self.algae_population_vector);
        egui::plot::Plot::new("Population Plot")
            .height(32.0)
            .data_aspect(1.0)
            .show(ui, |plot_ui| plot_ui
                .line(bacters_line)) // TODO find a way to do multiple lines
            .response
    }
}
