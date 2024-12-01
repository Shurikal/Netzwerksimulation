use std::vec;

use optimization::{linprog::solve, Consumer, Entity, Grid, Producer, Storage};


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EntityJson {
    pub name: String,
    pub cost_prod: Vec<f64>,
    pub power_prod: Vec<f64>,
    pub cost_cons: Vec<f64>,
    pub power_cons: Vec<f64>,
    
    pub entity_type: String,
}


fn main() {

    //let contents = std::fs::read_to_string("entities.json").unwrap();

    // read in the entities from a json file
    //let entities: Vec<EntityJson> = serde_json::from_str(&contents).unwrap();




    let grid = Grid::new(
        "Grid".to_string(),
        vec![10.0],
        vec![10.0],

        vec![10.0],
        vec![10.0],
        
    );

    let city = Consumer::new(
        vec![0.5,1.0],
        vec![1.0],
        vec![0.0],
        "City".to_string(),
    );

    let pv = Producer::new(
        vec![1.0,0.5],
        vec![1.0],
        vec![0.0],
        false,
        "PV".to_string(),
    );

    let storage = Storage::new(
        vec![1.0],
        vec![0.0],
        vec![1.0],
        vec![1.0],
        vec![1.0],
        vec![0.0],
        5.0,
        0.0,
        "Storage".to_string(),
    );

    let entities = vec![
        //Entity::Grid(grid),
        Entity::Consumer(city),
        Entity::Producer(pv),
        Entity::Storage(storage),
    ];

    let timesteps: usize = 10;

    let result = solve(entities, timesteps);

    match result {
        Ok(entities) => {
            let json: String = serde_json::to_string(&entities).unwrap();

            std::fs::write("result.json", json).unwrap();
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

}