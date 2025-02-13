# Simulation of a Simplified Electrical Grid

A simulation tool for modeling a simplified electrical grid with various types of power plants and consumers. The tool uses a linear programming solver to optimize costs for all connected entities.

## Features

The simulation supports the following entities:

- **Grid**: Manages excess energy by producing or consuming as needed.
- **Producer**: Generates energy with defined capacity and efficiency constraints.
- **Consumer**: Requires energy to be supplied at specific timesteps.
- **Storage**: Stores surplus energy for later use, with configurable charging and discharging rules.

## Entity Definitions

### Grid

The grid is defined by the following parameters:

- `name` (string): Grid identifier
- `cost_prod` (array): Production costs per timestep
- `power_prod` (array): Production capacities per timestep
- `cost_cons` (array): Consumption costs per timestep
- `power_cons` (array): Consumption capacities per timestep
- `entity_type` (string): Must be "Grid"

### Consumer

- `name` (string): Consumer identifier
- `power_cons` (array): Consumption demands per timestep
- `eff_cons` (array): Consumption efficiencies per timestep
- `cost_cons` (array): Consumption costs per timestep
- `entity_type` (string): Must be "Consumer"

### Producer

- `name` (string): Producer identifier
- `can_be_disabled` (boolean): Indicates whether the producer can be disabled
- `power_prod` (array): Production capacities per timestep
- `eff_prod` (array): Production efficiencies per timestep
- `cost_prod` (array): Production costs per timestep
- `entity_type` (string): Must be "Producer"

### Storage

- `name` (string): Storage identifier
- `power_prod` (array): Production capacities per timestep
- `eff_prod` (array): Production efficiencies per timestep
- `cost_prod` (array): Production costs per timestep
- `power_cons` (array): Consumption capacities per timestep
- `eff_cons` (array): Consumption efficiencies per timestep
- `cost_cons` (array): Consumption costs per timestep
- `storage_capacity` (float): Maximum energy storage capacity
- `start_capacity` (float): Initial stored energy
- `end_capacity` (float, optional): Final stored energy
- `storage_to_grid_allowed` (boolean): Whether storage can supply energy to the grid (default: `false`)
- `grid_to_storage_allowed` (boolean): Whether the grid can charge the storage (default: `false`)
- `entity_type` (string): Must be "Storage"

### Parameter Handling

All numerical parameters must be provided as arrays. The value for a given timestep is determined using the following approach:

```rust
let index = timestep % array.len();
let value = array[index];
return value;
```

### Defining Periodic Values

- **Single-value array (`length = 1`)**: Applies a constant value across all timesteps.
- **Hourly values (`length = 24`)**: Specifies a unique value for each hour of the day.
- **Custom periodicities**: Arrays of varying lengths can be used to define specific repeating patterns.

This flexibility allows for realistic modeling of energy consumption and production patterns.

## Build Instructions

Compile the project using Cargo:

```bash
cargo build --release
```

## Testing

Run the test suite with:

```bash
cargo test
```

## Usage

To use the solver, create a JSON file containing all the entities.
The file structure is as follows:

```json
{
    "entities": [
        {
            "name": "Grid",
            "cost_prod": [1.0],
            "power_prod": [10.0],
            "cost_cons": [-1.0],
            "power_cons": [10.0],
            "entity_type": "Grid"
        },
        ...
    ],
    "timesteps": 24
}
```

Then, run the solver with:

```bash
./target/release/solver < path_to_json > path_to_output
```

# Acknowledgements

This project depends on [good_lp](https://github.com/rust-or/good_lp) for formulating and solving linear programs.