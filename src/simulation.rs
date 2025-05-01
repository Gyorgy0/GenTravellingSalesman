use crate::app;

use app::{Individual, Town};


pub fn calculate_length(town_a: Town, town_b: Town) -> f32 {
    let town_a_pos = town_a.position;
    let town_b_pos = town_b.position;
    ((town_a_pos.x-town_b_pos.x).powi(2) + (town_a_pos.y-town_b_pos.y).powi(2)).sqrt()
}

pub fn generate_population(population: &mut Vec<Individual>, towns: Vec<Town>, population_number: usize) {

}

pub fn select_parents(arents: &mut Vec<Individual>, population: &mut Vec<Individual>) {
    
}

pub fn make_children(parents: Vec<Individual>, population: &mut Vec<Individual>) {

}