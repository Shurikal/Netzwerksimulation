use std::error::Error;

use chrono::{DateTime, Datelike};
use chrono_tz::Tz;
use good_lp::{
    clarabel, variable, variables, Constraint, Expression, Solution, SolverModel, Variable,
};

use crate::Entity;

pub fn solve(mut entities: Vec<Entity>, timesteps: usize) -> Result<Vec<Entity>, Box<dyn Error>> {
    let mut to_minimize: Expression = 0.into();

    let mut constraints: Vec<Constraint> = vec![];
    let mut problem_vars = variables!();

    for timestep in 0..timesteps {
        let mut node_eq: Expression = 0.into();

        for entity in entities.iter_mut() {
            match entity {
                Entity::Consumer(consumer) => {
                    let consumed = problem_vars.add(variable().min(0).max(1.0));

                    consumer.consumed_var.push(consumed);

                    // Kirchhoff
                    node_eq += consumed * -1.0 * consumer.get_power_cons(timestep)
                        / consumer.get_eff_cons(timestep);

                    // Consumers need the power demanded
                    constraints.push((1.0 * consumed).eq(1.0));

                    to_minimize += consumed
                        * consumer.get_cost_cons(timestep)
                        * consumer.get_power_cons(timestep);
                }
                Entity::Producer(producer) => {
                    let produced = problem_vars.add(variable().min(0).max(1.0));

                    producer.produced_var.push(produced);

                    if !producer.can_be_disabled {
                        constraints.push((1.0 * produced).eq(1.0));
                    }

                    node_eq += produced
                        * producer.get_power_prod(timestep)
                        * producer.get_eff_prod(timestep);
                    to_minimize += produced
                        * producer.get_cost_prod(timestep)
                        * producer.get_power_prod(timestep);
                }
                Entity::Storage(storage) => {
                    let consumed = problem_vars.add(variable().min(0).max(1.0));
                    let produced = problem_vars.add(variable().min(0).max(1.0));

                    storage.consumed_var.push(consumed);
                    storage.produced_var.push(produced);

                    let mut storage_eq: Expression = 0.into();

                    storage_eq += storage.start_capacity;
                    // storage balance
                    for j in 0..timestep {
                        storage_eq += storage.produced_var[j] * storage.get_eff_prod(j)
                            - storage.consumed_var[j] ;
                    }

                    constraints.push(storage_eq.geq(0));

                    node_eq += produced
                        * storage.get_power_prod(timestep)
                        * storage.get_eff_prod(timestep)
                        - consumed * storage.get_power_cons(timestep)
                            / storage.get_eff_cons(timestep);

                    to_minimize += consumed
                        * storage.get_cost_cons(timestep)
                        * storage.get_power_cons(timestep)
                        + produced
                            * storage.get_cost_prod(timestep)
                            * storage.get_power_prod(timestep);
                }
                Entity::Grid(grid) => {
                    let consumed = problem_vars.add(variable().min(0).max(1.0));
                    let produced = problem_vars.add(variable().min(0).max(1.0));

                    grid.consumed_var.push(consumed);
                    grid.produced_var.push(produced);

                    node_eq += produced * grid.get_power_prod(timestep)
                        - consumed * grid.get_power_cons(timestep);

                    to_minimize += consumed
                        * grid.get_cost_cons(timestep)
                        * grid.get_power_cons(timestep)
                        + produced * grid.get_cost_prod(timestep) * grid.get_power_prod(timestep);
                }
            }
        }

        constraints.push(node_eq.eq(0));
    }

    let solution = constraints.into_iter().fold(
        problem_vars.minimise(to_minimize).using(clarabel),
        |solution, constraint| solution.with(constraint),
    );

    let solution = solution.solve();

    match solution {
        Ok(_) => {
            let solution = solution.unwrap();

            for entity in entities.iter_mut() {
                match entity {
                    Entity::Consumer(consumer) => {
                        for (timestep, consumed_var) in consumer.consumed_var.iter().enumerate() {
                            consumer.consumed.push(
                                solution.value(*consumed_var) * consumer.get_power_cons(timestep),
                            );
                        }
                    }
                    Entity::Producer(producer) => {
                        for (i, produced_var) in producer.produced_var.iter().enumerate() {
                            producer
                                .produced
                                .push(solution.value(*produced_var) * producer.get_power_prod(i));
                        }
                    }
                    Entity::Storage(storage) => {
                        for (i, consumed_var) in storage.consumed_var.iter().enumerate() {
                            storage
                                .consumed
                                .push(solution.value(*consumed_var) * storage.get_power_cons(i));
                        }

                        for (i, produced_var) in storage.produced_var.iter().enumerate() {
                            storage
                                .produced
                                .push(solution.value(*produced_var) * storage.get_power_prod(i));
                        }
                    }
                    Entity::Grid(grid) => {
                        for (i, consumed_var) in grid.consumed_var.iter().enumerate() {
                            grid.consumed
                                .push(solution.value(*consumed_var) * grid.get_power_cons(i));
                        }

                        for (i, produced_var) in grid.produced_var.iter().enumerate() {
                            grid.produced
                                .push(solution.value(*produced_var) * grid.get_power_prod(i));
                        }
                    }
                }
            }
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    }

    return Ok(entities);
}
