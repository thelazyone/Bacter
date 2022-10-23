use core::cell::Cell; // for the Bacter Cell trait.
mod data_visualization;   
 
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Creating the Universe as a Petri Dish.
    // Skipping the serialization (since it's fairly heavy). Might re-enable in the future.
    #[serde(skip)]
    // TODO

    // Plots Data
    simulation_data : data_visualization::DataVisualization,

    // Parameters for the simulation
    petri_size : u32, // Default is 500 px
    algae_growth_ratio: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            simulation_data : data_visualization::DataVisualization::default(),

            // Example stuff:
            petri_size : 500,
            algae_growth_ratio : 1.,
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
            simulation_data, 
            petri_size, 
            algae_growth_ratio } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

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

            // TODO TBR
            //
            // Edit field
            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(label);
            // });
            // 
            // Slider
            ui.add(egui::Slider::new(petri_size, 0..=1000).text("Simulation Size (side)"));
            ui.add(egui::Slider::new(algae_growth_ratio, 0.0..=5.).text("Algae Growth"));

            // // Parameters:
            // ui.horizontal(|ui| {
            //     ui.label("Simulation Size (side): ");
            //     ui.text_edit_singleline(petri_size);
            // });
            // ui.horizontal(|ui| {
            //     ui.label("Algae Growth: ");
            //     ui.text_edit_singleline(algae_growth_ratio);
            // });
            

            // Start Simulation Button
            if ui.button("Start Simulation").clicked() {
                // Starting the simulation async
                // TODO use SM commands or a polled flag
            }

            // Start Simulation Button
            if ui.button("Pause Simulation").clicked() {
                // Starting the simulation async
                // TODO use SM commands or a polled flag
            }

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
            // The central panel the region left after adding TopPanel's and SidePanel's

            //ui.heading("eframe template");
            // ui.hyperlink("https://github.com/emilk/eframe_template");
            // ui.add(egui::github_link_file!(
            //     "https://github.com/emilk/eframe_template/blob/master/",
            //     "Source code."
            // ));

            egui::Window::new("Populations")
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label("Bacters and Algae population.");
                    self.simulation_data.population_plot(ui);
            });

            egui::warn_if_debug_build(ui);
        });
    }
}
