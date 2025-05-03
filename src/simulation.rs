use std::ops::DerefMut;

use crate::app;
use egui::{emath::OrderedFloat, Pos2};
use itertools::Itertools;

use app::{Individual, Town};

pub fn calculate_length(town_a: &Town, town_b: &Town) -> f32 {
    let town_a_pos = town_a.position;
    let town_b_pos = town_b.position;
    ((town_a_pos.x.abs() - town_b_pos.x.abs()).powi(2)
        + (town_a_pos.y.abs() - town_b_pos.y.abs()).powi(2))
    .sqrt()
}

pub fn calculate_travelled_distance(travelled_towns: &Vec<Town>) -> OrderedFloat<f32> {
    let mut dist = OrderedFloat(0_f32);
    for i in 0..travelled_towns.len() {
        dist.0 += calculate_length(
            &travelled_towns[i],
            &travelled_towns[(i + 1) % travelled_towns.len()],
        );
    }
    dist
}

pub fn generate_towns(town_min_dist: f32, town_max_dist: f32, n_o_towns: usize) -> Vec<Town> {
    let mut towns: Vec<Town> = vec![];
    for i in 0..n_o_towns {
        let rnd_offset_x = rand::random_range(town_min_dist..=town_max_dist)
            * rand::random_range(-1_i8..=0_i8).signum() as f32;
        let rnd_offset_y = rand::random_range(town_min_dist..=town_max_dist)
            * rand::random_range(-1_i8..=0_i8).signum() as f32;
        if towns.is_empty() {
            towns.push(Town::new(
                format!("{}.", i + 1),
                Pos2::new(0.0 + rnd_offset_x, 0.0 + rnd_offset_y),
            ));
        } else {
            let mut new_pos = Pos2::new(
                towns[i - 1].position.x + rnd_offset_x,
                towns[i - 1].position.y + rnd_offset_y,
            );
            while towns.iter().any(|t| t.position == new_pos) {
                let rnd_offset_x = rand::random_range(town_min_dist..=town_max_dist)
                    * rand::random_range(-1_i8..=0_i8).signum() as f32;
                let rnd_offset_y = rand::random_range(town_min_dist..=town_max_dist)
                    * rand::random_range(-1_i8..=0_i8).signum() as f32;
                new_pos = Pos2::new(
                    towns[rand::random_range(0..i)].position.x + rnd_offset_x,
                    towns[rand::random_range(0..i)].position.y + rnd_offset_y,
                );
            }
            towns.push(Town::new(format!("{}.", i + 1), new_pos));
        }
    }
    towns
}

pub fn genetic_search(
    towns: &Vec<Town>,
    population: &mut Vec<Individual>,
    mutation_chance: f32,
    min_improvement: f32,
    evolution_started: &mut bool,
) {

}

pub fn generate_population(towns: &Vec<Town>, population_number: usize) -> Vec<Individual> {
    let mut population: Vec<Individual> = vec![];
    let mut individual: Individual = Individual {
        travelled_towns: vec![],
        travelled_distance: OrderedFloat::from(0_f32),
        actual_town: String::new(),
    };
    for i in 0..population_number {
        let mut remaining_towns = towns.clone();
        for j in 0..towns.len() {
            let rnd_index = rand::random_range(0..remaining_towns.len());
            individual
                .travelled_towns
                .push(remaining_towns.get(rnd_index).unwrap().clone());
            remaining_towns.remove(rnd_index);
        }
        individual.travelled_distance = calculate_travelled_distance(&individual.travelled_towns);
        population.push(individual.clone());
    }
    // Which towns were visited + travelled distance
    // Mely városok voltak meglátogatvas + megtett távolság
    // println!("{:?}", individual.travelled_towns.clone());
    // println!("{:?}", individual.travelled_distance.0.clone());
    population
}

pub fn select_parents(population: &mut Vec<Individual>) -> Vec<&mut Individual> {
    let mut ranked_individuals = population
        .into_iter()
        .sorted_by_key(|f| f.travelled_distance)
        .map(|individual| individual)
        .collect_vec();
    let parents:Vec<&mut Individual> = ranked_individuals.split_off(ranked_individuals.len()/2_usize);
    parents

}

pub fn make_children(parents: Vec<Individual>, population_number: usize) -> Vec<Individual> {
    let mut population = parents.clone();
    let mut children:Vec<Individual> = vec![];
    while population.len() < population_number {
        let first_parent_index = rand::random_range(0..parents.len());
        let mut second_parent_index = rand::random_range(0..parents.len());
        while first_parent_index == second_parent_index {
            second_parent_index = rand::random_range(0..parents.len());
        }
        let cross_over_start: usize = rand::random_range(0..parents[0].travelled_towns.len());
        let cross_over_end:usize = rand::random_range(cross_over_start..parents[0].travelled_towns.len());
        let mut new_path:Vec<Town> = vec![];
        for i in 0..parents[0].travelled_towns.len() {
            new_path.push(parents[first_parent_index].travelled_towns[i].clone());
        }
        for i in cross_over_start..cross_over_end {
            new_path[parents[first_parent_index].travelled_towns.iter().find_position(|gene| gene == format!("{}.", i+1))]
        }
    }
    population
}
