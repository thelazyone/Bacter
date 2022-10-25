use std::time::Duration;

use super::cell::cell::Cell as bacter_cell; // for the Bacter Cell trait.
use super::cell::cell::Float2D as bacter_float2D; // for the Bacter Cell trait.
use super::cell::dish::Dish as bacter_dish; // for the Bacter Cell trait.
use crate::data_visualization::DataVisualization;


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Creating the Universe as a Petri Dish.
    // Skipping the serialization (since it's fairly heavy). Might re-enable in the future.

    // Petri Dish and the model
    #[serde(skip)]
    dish: bacter_dish,
    #[serde(skip)]
    simulation_data : DataVisualization,
    is_simulation_running : bool,

    // Parameters for the simulation
    petri_size : u32, // Default is 500 px
    algae_growth_ratio: f32,

    // Debugging message:
    last_message : String
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            dish : bacter_dish::new(bacter_float2D{x: 0  as f64, y: 0 as f64}, 0),
            simulation_data : DataVisualization::default(),
            is_simulation_running : false,

            // Default fields:
            petri_size : 500,
            algae_growth_ratio : 1.,

            last_message: "".to_owned(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        // If no previous state has been found, creating a default one.
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { 
            dish,
            simulation_data, 
            is_simulation_running,
            petri_size, 
            algae_growth_ratio,
            last_message } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // Updating the plot info
        if let Some(last_iteration) = self.simulation_data.iteration.last().copied(){
            if self.dish.get_iteration() > last_iteration as i32{


                self.simulation_data.iteration.push(self.dish.get_iteration() as u32);
                self.simulation_data.bacters_population.push(self.dish.bacters.len() as u32);
                self.simulation_data.algae_population.push(self.dish.algae.len() as u32);

                self.last_message = 
                    format!("Adding Iteration {}, vect size {}",
                        self.dish.get_iteration(),
                        self.simulation_data.iteration.len()
                        ).to_owned();
            }
        }
        else{

            self.simulation_data.iteration.push(self.dish.get_iteration() as u32);
            self.simulation_data.bacters_population.push(self.dish.bacters.len() as u32);
            self.simulation_data.algae_population.push(self.dish.algae.len() as u32);

            self.last_message = format!("First Iteration!").to_owned();
        }

        // This shows only when compiled for desktop app!
        #[cfg(not(target_arch = "wasm32"))] 
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });


        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Bacter");

            ui.add(egui::Slider::new(&mut self.petri_size, 0..=1000).text("Simulation Size (side)"));
            ui.add(egui::Slider::new(&mut self.algae_growth_ratio, 0.0..=5.).text("Algae Growth"));            

            // Start Simulation Button
            if ui.button("Start Simulation").clicked() {
                
                // Updating the thing:
                self.dish = bacter_dish::new(bacter_float2D{
                    x: self.petri_size as f64,
                    y: self.petri_size as f64},
                    100);
                
                self.is_simulation_running = true;



                // Starting the simulation async
                // tokio::spawn(
                //     Interval::new_interval(Duration::from_millis(500))
                //     .for_each(|_| {
                //         self.dish.tick(1);
                //         Ok(())
                //     })
                //     .map_err(|_| ()),
                // );

                // TODO use SM commands or a polled flag
            }

            // Step Simulation Button - TODO TBR!
            if ui.button("Step Simulation").clicked() {

                for _ in 0..100 { 

                self.dish.simulation_step();
                }
            }

            // Pause Simulation Button
            if ui.button("Pause Simulation").clicked() {
                // Starting the simulation async
                // TODO use SM commands or a polled flag
            }

            // Debug Log:
            ui.label(self.last_message.clone());


            // Keeping that because they deserve it.
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by the amazing ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {

            // Populations plot
            egui::Window::new("Populations")
                .resizable(true)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label("Bacters and Algae population.");
                    self.simulation_data.population_plot(ui);
            });

            egui::warn_if_debug_build(ui);
        });
    }
}

