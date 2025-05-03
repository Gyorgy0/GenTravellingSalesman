use std::f32;

use egui::{emath::OrderedFloat, Align2, Pos2, RichText, Widget};
use egui_plot::{Arrows, PlotPoint, PlotPoints, Points, Text};

use crate::simulation::{generate_population, generate_towns, genetic_search};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GenTravellingSalesmanApp {
    n_o_towns: usize,
    manual_town_input: bool,
    #[serde(skip)]
    evolution_started: bool,
    #[serde(skip)]
    towns: Vec<Town>,
    population_number: usize,
    #[serde(skip)]
    population: Vec<Individual>,
    #[serde(skip)]
    parents: Vec<Individual>,
    town_min_dist: f32,
    town_max_dist: f32,
    generation_counter: usize,
    #[serde(skip)]
    best_from_each_gen: Vec<Individual>,
    #[serde(skip)]
    selected_individual: Option<Individual>,
    mutation_chance: f32,
    //min_improvement: f32,
}

impl Default for GenTravellingSalesmanApp {
    fn default() -> Self {
        Self {
            n_o_towns: 0_usize,
            manual_town_input: false,
            evolution_started: false,
            towns: vec![],
            population_number: 0_usize,
            population: vec![],
            parents: vec![],
            town_min_dist: 1_f32,
            town_max_dist: 1.1_f32,
            generation_counter: 0_usize,
            best_from_each_gen: vec![],
            selected_individual: Option::None,
            mutation_chance: 0.02,
            //min_improvement: 0.05,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Individual {
    pub travelled_towns: Vec<Town>,
    pub travelled_distance: OrderedFloat<f32>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Town {
    pub name: String,
    pub position: Pos2,
}

impl Town {
    pub fn new(name: String, position: Pos2) -> Self {
        Self { name, position }
    }
}

impl GenTravellingSalesmanApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        cc.egui_ctx.set_pixels_per_point(1.5);
        /*if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }*/
        Default::default()
    }
}

impl eframe::App for GenTravellingSalesmanApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Window::new("Options (beállítások)").show(ctx, |ui| {
                let mut evolution_ui = egui::UiBuilder::new();
                if self.evolution_started {
                    evolution_ui = evolution_ui.disabled();
                }
                ui.scope_builder(evolution_ui, |ui| {
                    ui.label("Number of towns (városok száma):");
                egui::widgets::Slider::new(&mut self.n_o_towns, 0_usize..=512_usize).ui(ui);
                let mut manual_input_ui = egui::UiBuilder::new();
                if self.manual_town_input {
                    manual_input_ui = manual_input_ui.disabled();
                }
                ui.separator();
                ui.label("Town options (városok beállításai):");
                ui.separator();
                ui.scope_builder(manual_input_ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                "Minimum distance between towns\n(városok közti minimális távolság):",
                            );
                            egui::widgets::Slider::new(&mut self.town_min_dist, 0_f32..=256_f32).ui(ui);
                        });
                        ui.vertical(|ui| {
                            ui.label(
                                "Maximum distance between towns\n(városok közti maximális távolság):",
                            );
                            egui::widgets::Slider::new(
                                &mut self.town_max_dist,
                                self.town_min_dist..=512_f32,
                            )
                            .ui(ui);
                        });
                    });
                    if ui.button("Generate").clicked() {
                        self.towns.clear();
                        self.best_from_each_gen.clear();
                    self.selected_individual = Option::None;
                        self.towns = generate_towns(self.town_min_dist, self.town_max_dist, self.n_o_towns);
                    }
                });
                // Plus feature idea: the user can specify towns, town names and their coordinates
                // Plusz funkció ötlet: a felhasználó megadhatja a városok nevét, illetve koordinátáit
                //ui.checkbox(&mut self.manual_town_input, "Manual town setup (városok kézi megadása)");
                ui.separator();
                ui.label("Maximum population (populáció nagysága):");
                egui::widgets::Slider::new(&mut self.population_number, 1..=512_usize)
                    .ui(ui);
                ui.label("Mutation chance (mutáció esélye):");
                egui::widgets::Slider::new(&mut self.mutation_chance, 0_f32..=1_f32)
                    .ui(ui);
                /*ui.separator();
                ui.label("Stopping condition (megállási feltétel):");
                ui.separator();
                ui.label("Improvement margin (javulási küszöb):");
                egui::widgets::Slider::new(&mut self.min_improvement, 0_f32..=1_f32)
                    .ui(ui);*/
                ui.separator();
                let empty_warn_popup = ui.make_persistent_id("no_towns");
            let start_button =  ui.button("Start");
                if start_button.clicked() && !self.towns.is_empty(){
                    self.best_from_each_gen.clear();
                    self.selected_individual = Option::None;
                    self.evolution_started = true;
                    self.population = generate_population(&self.towns, self.population_number);
                    genetic_search( &mut self.population,&mut self.best_from_each_gen, self.mutation_chance, /*self.min_improvement,*/ self.population_number,&mut self.evolution_started);
                }
                else if start_button.clicked() && self.towns.is_empty() {
                    ui.memory_mut(|mem| mem.toggle_popup(empty_warn_popup))
                }
                egui::popup::popup_above_or_below_widget(ui, empty_warn_popup, &start_button, egui::AboveOrBelow::Above, egui::PopupCloseBehavior::CloseOnClickOutside, |ui| {
                    ui.set_min_width(200.0);
                    ui.label("You need to generate the towns first!");
                    ui.label("Ki kell generálnod a városokat először!");});
                });
            });
            if !self.evolution_started && !self.best_from_each_gen.is_empty() {
                egui::Window::new("Best of each generation (minden generáció legjobbja)").show(ctx, |ui| {
                    egui::ScrollArea::both().max_width(f32::INFINITY).show(ui, |ui| {
                        for i in 0..self.best_from_each_gen.len() {
                        ui.horizontal(|ui|{
                            if ui.button(format!("{}. gen", i+1)).clicked() {
                                self.selected_individual = Option::Some(self.best_from_each_gen[i].clone());
                            }
                            ui.label(format!("Length (hossz): {}", self.best_from_each_gen[i].travelled_distance.0));
                        });
                        let mut path = String::new();
                        for j in 0..self.best_from_each_gen[i].travelled_towns.len() {
                            path += &self.best_from_each_gen[i].travelled_towns[j].name.to_string();
                            if (j+1) != self.best_from_each_gen[i].travelled_towns.len() {
                                path += " => "
                            }
                        }
                        ui.label(format!("Path (megtett út): {}", path));
                    }
                    });
                });
            }
            egui_plot::Plot::new("")
                .allow_drag(true)
                .show_grid(false)
                .show_y(false)
                .show_x(false)
                .show(ui, |plot_ui| {
                    // Town visualization - Városok vizualizálása
                    for i in 0..self.towns.len() {
                        plot_ui.text(
                            Text::new(
                                &self.towns[i].name,
                                PlotPoint::new(self.towns[i].position.x as f64, self.towns[i].position.y),
                                RichText::new(format!("{} város", self.towns[i].name)).strong().size(10.0),
                            )
                            .highlight(true)
                            .anchor(Align2::CENTER_BOTTOM),
                        );
                        plot_ui.points(
                            Points::new(&self.towns[i].name, PlotPoints::new(vec![[self.towns[i].position.x as f64, self.towns[i].position.y as f64]]))
                                .highlight(true)
                                .radius(2.0),
                        );
                    }
                    if self.selected_individual.is_some() {
                        for i in 0..self.selected_individual.clone().unwrap().travelled_towns.len() {
                            let towns = self.selected_individual.clone().unwrap().travelled_towns;
                            plot_ui.arrows(
                                Arrows::new(
                                    "",
                                    PlotPoints::new(vec![[towns[i].position.x as f64, towns[i].position.y as f64]]),
                                    PlotPoints::new(vec![[towns[(i+1)%towns.len()].position.x as f64, towns[(i+1)%towns.len()].position.y as f64]]),
                                )
                                .tip_length(30.0),
                            );
                        }
                    }
                });
        });
    }
}
