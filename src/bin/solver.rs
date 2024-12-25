use std::{io::{self, Read}, vec};

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

    pub consumed : Option<Vec<f64>>,
    pub produced : Option<Vec<f64>>,
    
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
    stdin_lock.read_to_string(&mut input).expect("Failed to read from stdin");

    let solver_json: SolverJson = serde_json::from_str(&input).unwrap();

    let mut entities: Vec<Entity> = vec![];

    for entity in solver_json.entities {
        match entity.entity_type.as_str() {
            "Grid" => {
                let grid = Grid::new(
                    entity.name,
                    entity.cost_prod.unwrap(),
                    entity.power_prod.unwrap(),
                    entity.cost_cons.unwrap(),
                    entity.power_cons.unwrap(),
                );
                entities.push(Entity::Grid(grid));
            }
            "Consumer" => {
                let consumer = Consumer::new(
                    entity.power_cons.unwrap(),
                    entity.eff_cons.unwrap(),
                    entity.cost_cons.unwrap(),
                    entity.name,
                );
                entities.push(Entity::Consumer(consumer));
            }
            "Producer" => {
                let producer = Producer::new(
                    entity.power_prod.unwrap(),
                    entity.eff_prod.unwrap(),
                    entity.cost_prod.unwrap(),
                    entity.can_be_disabled.unwrap(),
                    entity.name,
                );
                entities.push(Entity::Producer(producer));
            }
            "Storage" => {
                let storage = Storage::new(
                    entity.power_prod.unwrap(),
                    entity.cost_prod.unwrap(),
                    entity.eff_cons.unwrap(),
                    entity.power_cons.unwrap(),
                    entity.eff_prod.unwrap(),
                    entity.cost_cons.unwrap(),
                    entity.storage_capacity.unwrap(),
                    entity.start_capacity.unwrap(),
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