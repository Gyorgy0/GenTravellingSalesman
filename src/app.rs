use std::collections::HashMap;

use egui::Pos2;
use serde::{Deserialize, Serialize};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GenTravellingSalesmanApp {
    // Example stuff:
    number_of_towns: usize,
    towns: Vec<Town>,
    population_number: u32,
    population: Vec<Individual>,
    parents: Vec<Individual>,
    town_min_dist: f32,
    town_max_dist: f32,
    generation_counter: usize,
    min_improvement: f32,
}

impl Default for GenTravellingSalesmanApp {
    fn default() -> Self {
        Self {
            number_of_towns: 0_usize,
            towns: vec![],
            population_number: 0_u32,
            population: vec![],
            parents: vec![],
            town_min_dist: 1_f32,
            town_max_dist: 1.1_f32,
            generation_counter: 0_usize,
            min_improvement: 0.01,
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Individual {
    pub travelled_towns: Vec<Town>,
    pub travelled_distance: f32,
    pub actual_town: String,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Town {
    pub name: String,
    pub position: Pos2,
    pub visited: bool,
}

impl Town {
    pub fn new(&mut self, name: String, position: Pos2) -> Self {
        Self {
            name: name,
            position: position,
            visited: false,
        }
    }
}

impl GenTravellingSalesmanApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

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
        egui::CentralPanel::default().show(ctx, |ui| {});
    }
}
