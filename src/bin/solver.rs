use std::{
    io::{self, Read},
    vec,
};

use optimization::{linprog::solve, Consumer, Entity, Grid, Producer, Storage};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EntityJson {
    pub name: String,
    pub cost_prod: Option<Vec<f64>>,
    pub power_prod: Option<Vec<f64>>,
    pub cost_cons: Option<Vec<f64>>,
    pub power_cons: Option<Vec<f64>>,
    pub eff_cons: Option<Vec<f64>>,
    pub eff_prod: Option<Vec<f64>>,
    pub can_be_disabled: Option<bool>,
    pub storage_capacity: Option<f64>,
    pub start_capacity: Option<f64>,

    pub consumed: Option<Vec<f64>>,
    pub produced: Option<Vec<f64>>,

    pub entity_type: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SolverJson {
    pub entities: Vec<EntityJson>,
    pub timesteps: usize,
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut input = String::new();
    stdin_lock
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");

    let solver_json: SolverJson = serde_json::from_str(&input).unwrap();

    let mut entities: Vec<Entity> = vec![];

    for entity in solver_json.entities {
        match entity.entity_type.as_str() {
            "Grid" => {
                let grid = Grid::new(
                    entity
                        .cost_cons
                        .expect(format!("{} is missing cost_cons", entity.name).as_str()),
                    entity
                        .power_cons
                        .expect(format!("{} is missing power_cons", entity.name).as_str()),
                    entity
                        .cost_prod
                        .expect(format!("{} is missing cost_prod", entity.name).as_str()),
                    entity
                        .power_prod
                        .expect(format!("{} is missing power_prod", entity.name).as_str()),
                    entity.name,
                );
                entities.push(Entity::Grid(grid));
            }
            "Consumer" => {
                let consumer = Consumer::new(
                    entity
                        .power_cons
                        .expect(format!("{} is missing power_cons", entity.name).as_str()),
                    entity
                        .eff_cons
                        .expect(format!("{} is missing eff_cons", entity.name).as_str()),
                    entity
                        .cost_cons
                        .expect(format!("{} is missing cost_cons", entity.name).as_str()),
                    entity.name,
                );
                entities.push(Entity::Consumer(consumer));
            }
            "Producer" => {
                let producer = Producer::new(
                    entity
                        .cost_prod
                        .expect(format!("{} is missing cost_prod", entity.name).as_str()),
                    entity
                        .eff_prod
                        .expect(format!("{} is missing eff_prod", entity.name).as_str()),
                    entity
                        .power_prod
                        .expect(format!("{} is missing power_prod", entity.name).as_str()),
                    entity
                        .can_be_disabled
                        .expect(format!("{} is missing can_be_disabled", entity.name).as_str()),
                    entity.name,
                );
                entities.push(Entity::Producer(producer));
            }
            "Storage" => {
                let storage = Storage::new(
                    entity
                        .cost_cons
                        .expect(format!("{} is missing cost_cons", entity.name).as_str()),
                    entity
                        .eff_cons
                        .expect(format!("{} is missing eff_cons", entity.name).as_str()),
                    entity
                        .power_cons
                        .expect(format!("{} is missing power_cons", entity.name).as_str()),
                    entity
                        .cost_prod
                        .expect(format!("{} is missing cost_prod", entity.name).as_str()),
                    entity
                        .eff_prod
                        .expect(format!("{} is missing eff_prod", entity.name).as_str()),
                    entity
                        .power_prod
                        .expect(format!("{} is missing power_prod", entity.name).as_str()),
                    entity
                        .storage_capacity
                        .expect(format!("{} is missing storage_capacity", entity.name).as_str()),
                    entity
                        .start_capacity
                        .expect(format!("{} is missing start_capacity", entity.name).as_str()),
                    entity.name,
                );
                entities.push(Entity::Storage(storage));
            }
            _ => {
                println!("Unknown entity type: {}", entity.entity_type);
            }
        }
    }

    let timesteps: usize = solver_json.timesteps;

    let result = solve(entities, timesteps);

    match result {
        Ok(entities) => {
            let json: String = serde_json::to_string(&entities).unwrap();
            println!("{}", json);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
