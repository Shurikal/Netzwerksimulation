use optimization::{linprog::solve, Consumer, Entity, Grid, Producer, Storage};


#[test]
fn not_connected_grid() {
    let mut entities: Vec<Entity> = vec![];
    let timesteps = 4;

    entities.push(Entity::Grid(Grid::new(
        vec![0.0],
        vec![1.0],
        vec![0.0],
        vec![1.0],
        "grid".to_string(),
    )));
    
    let result = solve(entities, timesteps);
    assert_eq!(result.is_ok(), true);

    let grid = match result.unwrap().pop().unwrap() {
        Entity::Grid(grid) => grid,
        _ => panic!("Expected Grid"),
    };

    assert_eq!(grid.consumed, vec![0.0, 0.0, 0.0, 0.0]);
    assert_eq!(grid.produced, vec![0.0, 0.0, 0.0, 0.0]);
}


#[test]
fn grid_and_consumer() {
    let mut entities: Vec<Entity> = vec![];
    let timesteps = 4;

    entities.push(Entity::Grid(Grid::new(
        vec![0.0],
        vec![1.0],
        vec![0.0],
        vec![1.0],
        "grid".to_string(),
    )));
    entities.push(Entity::Consumer(Consumer::new(
        vec![0.0, 1.0],
        vec![1.0],
        vec![0.0],
        "consumer".to_string(),
    )));
    
    let result = solve(entities, timesteps);
    assert_eq!(result.is_ok(), true);

    let mut unwrapped_result = result.unwrap();

    let consumer = match unwrapped_result.pop().unwrap() {
        Entity::Consumer(consumer) => consumer,
        _ => panic!("Expected Consumer"),
    };

    let grid = match unwrapped_result.pop().unwrap() {
        Entity::Grid(grid) => grid,
        _ => panic!("Expected Grid"),
    };

    assert_eq!(consumer.consumed, vec![0.0, 1.0, 0.0, 1.0]);
    assert_eq!(grid.consumed, vec![0.0, 0.0, 0.0, 0.0]);
    assert_eq!(grid.produced, vec![0.0, 1.0, 0.0, 1.0]);
}

#[test]
fn consumer_and_storage() {
    let mut entities: Vec<Entity> = vec![];
    let timesteps = 4;

    entities.push(Entity::Storage(Storage::new(
        vec![0.0],
        vec![1.0],
        vec![1.0],
        vec![0.0],
        vec![1.0],
        vec![1.0],
        20.0,
        20.0,
        "storage".to_string(),
    )));
    entities.push(Entity::Consumer(Consumer::new(
        vec![0.0, 1.0],
        vec![1.0],
        vec![0.0],
        "consumer".to_string(),
    )));
    
    let result = solve(entities, timesteps);
    assert_eq!(result.is_ok(), true);

    let mut unwrapped_result = result.unwrap();

    let consumer = match unwrapped_result.pop().unwrap() {
        Entity::Consumer(consumer) => consumer,
        _ => panic!("Expected Consumer"),
    };

    let storage = match unwrapped_result.pop().unwrap() {
        Entity::Storage(storage) => storage,
        _ => panic!("Expected Storage"),
    };

    assert_eq!(consumer.consumed, vec![0.0, 1.0, 0.0, 1.0]);
    assert_eq!(storage.produced, vec![0.0, 1.0, 0.0, 1.0]);

    assert_eq!(storage.stored, vec![20.0, 19.0, 19.0, 18.0]);
}