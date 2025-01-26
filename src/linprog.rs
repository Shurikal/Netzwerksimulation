use std::error::Error;

use good_lp::{highs, variable, variables, Constraint, Expression, Solution, SolverModel};

use crate::Entity;

pub fn solve(mut entities: Vec<Entity>, timesteps: usize) -> Result<Vec<Entity>, Box<dyn Error>> {
    let mut to_minimize: Expression = 0.into();

    let mut constraints: Vec<Constraint> = vec![];
    let mut problem_vars = variables!();

    for timestep in 0..timesteps {
        let mut node_eq: Expression = 0.into();

        let mut produced_eq: Expression = 0.into();
        let mut consumed_eq: Expression = 0.into();

        let mut produced_storage_eq: Expression = 0.into();
        let mut consumed_storage_eq: Expression = 0.into();

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

                    consumed_eq += 1.0 *consumed * consumer.get_power_cons(timestep) / consumer.get_eff_cons(timestep);

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

                    produced_eq += 1.0 * produced * producer.get_power_prod(timestep) / producer.get_eff_prod(timestep);

                    node_eq += produced
                        * producer.get_power_prod(timestep)
                        * producer.get_eff_prod(timestep);
                    to_minimize += produced
                        * producer.get_cost_prod(timestep)
                        * producer.get_power_prod(timestep);
                }
                Entity::Storage(storage) => {
                    let consumed = problem_vars.add(
                        variable()
                            .min(0)
                            .max(1.0)
                            .name(format!("{}-{}-c", storage.name, timestep)),
                    );
                    let produced = problem_vars.add(
                        variable()
                            .min(0)
                            .max(1.0)
                            .name(format!("{}-{}-p", storage.name, timestep)),
                    );

                    storage.consumed_var.push(consumed);
                    storage.produced_var.push(produced);

                    let mut storage_min_eq: Expression = 0.into();
                    let mut storage_max_eq: Expression = 0.into();

                    let producing = problem_vars.add(variable().binary());
                    storage.producing_var.push(producing);

                    // Constraints to enforce mutual exclusivity
                    constraints.push((1.0 * produced).leq(1.0 * producing)); // produced <= binary_var
                    constraints.push((1.0 * consumed).leq(1.0 * (1.0 - producing))); // consumed <= 1 - binary_var

                    storage_min_eq += storage.start_capacity;
                    storage_max_eq += storage.start_capacity;

                    consumed_eq += 1.0 * consumed * storage.get_power_cons(timestep) / storage.get_eff_cons(timestep);
                    produced_eq += 1.0 * produced * storage.get_power_prod(timestep) / storage.get_eff_prod(timestep);

                    if !storage.storage_to_grid_allowed {
                        produced_storage_eq += 1.0 * produced * storage.get_power_prod(timestep) / storage.get_eff_prod(timestep);
                    }
                    if !storage.grid_to_storage_allowed {
                        consumed_storage_eq += 1.0 * consumed * storage.get_power_cons(timestep) / storage.get_eff_cons(timestep);
                    }

                    // storage balance
                    for j in 0..timestep + 1 {
                        storage_min_eq += storage.consumed_var[j]
                            * storage.get_eff_cons(j)
                            * storage.get_power_cons(timestep)
                            - storage.produced_var[j] * storage.get_power_prod(timestep);

                        storage_max_eq += storage.consumed_var[j]
                            * storage.get_eff_cons(j)
                            * storage.get_power_cons(timestep)
                            - storage.produced_var[j] * storage.get_power_prod(timestep);
                    }

                    if storage.end_capacity.is_some() && timestep == timesteps - 1 {
                        let mut end_storage_eq: Expression = 0.into();
                        end_storage_eq += storage.start_capacity;
                        for j in 0..timestep + 1 {
                            end_storage_eq += storage.consumed_var[j]
                                * storage.get_eff_cons(j)
                                * storage.get_power_cons(timestep)
                                - storage.produced_var[j] * storage.get_power_prod(timestep);
                        }
                        constraints.push(end_storage_eq.eq(storage.end_capacity.unwrap()));
                    }

                    constraints.push(storage_min_eq.geq(0));
                    constraints.push(storage_max_eq.leq(storage.storage_capacity));

                    node_eq += produced
                        * storage.get_power_prod(timestep)
                        * storage.get_eff_prod(timestep)
                        - consumed * storage.get_power_cons(timestep) / storage.get_eff_cons(timestep);

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

                    let producing = problem_vars.add(variable().binary());
                    grid.producing_var.push(producing);

                    // Constraints to enforce mutual exclusivity
                    constraints.push((1.0 * produced).leq(1.0 * producing)); // produced <= binary_var
                    constraints.push((1.0 * consumed).leq(1.0 * (1.0 - producing))); // consumed <= 1 - binary_var

                    node_eq += produced * grid.get_power_prod(timestep)
                        - consumed * grid.get_power_cons(timestep);

                    to_minimize += consumed
                        * grid.get_cost_cons(timestep)
                        * grid.get_power_cons(timestep)
                        + produced * grid.get_cost_prod(timestep) * grid.get_power_prod(timestep);
                }
            }
        }

        constraints.push(node_eq.eq(0).set_name(format!("Kirchhoff @{}", timestep)));

        constraints.push(produced_eq.geq(consumed_storage_eq).set_name(format!("Storage @{}", timestep)));
        constraints.push(consumed_eq.geq(produced_storage_eq).set_name(format!("Storage @{}", timestep)));
    }

    let solution = constraints.into_iter().fold(
        problem_vars.minimise(to_minimize).using(highs),
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
                        let n_entries = storage.produced_var.len();

                        let mut stored = storage.start_capacity;

                        for i in 0..n_entries {
                            storage.consumed.push(
                                solution.value(storage.consumed_var[i]) * storage.get_power_cons(i),
                            );

                            storage.produced.push(
                                solution.value(storage.produced_var[i])
                                    * storage.get_power_prod(i)
                                    * storage.get_eff_prod(i),
                            );

                            stored += storage.consumed[i] * storage.get_eff_cons(i)
                                - storage.produced[i] / storage.get_eff_prod(i);
                            storage.stored.push(stored);
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
