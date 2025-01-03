use good_lp::Variable;
use serde::Serialize;

pub mod linprog;

fn check_eff_vec(eff: &Vec<f64>) {
    for eff in eff.iter() {
        if *eff < 0.0 || *eff > 1.0 {
            panic!("Efficiency must be between 0 and 1");
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Consumer {
    pub power_cons: Vec<f64>,
    pub eff_cons: Vec<f64>,
    pub cost_cons: Vec<f64>,
    pub name: String,

    #[serde(skip_serializing)]
    pub consumed_var: Vec<Variable>,
    pub consumed: Vec<f64>,

    pub entity_type: String,
}

impl Consumer {
    pub fn new(
        power_cons: Vec<f64>,
        eff_cons: Vec<f64>,
        cost_cons: Vec<f64>,
        name: String,
    ) -> Self {
        check_eff_vec(&eff_cons);
        Consumer {
            power_cons,
            eff_cons,
            name,
            cost_cons,
            consumed_var: vec![],
            consumed: vec![],
            entity_type: "Consumer".to_string(),
        }
    }

    pub fn get_power_cons(&self, timestep: usize) -> f64 {
        let index = timestep % self.power_cons.len();
        self.power_cons[index]
    }

    pub fn get_eff_cons(&self, timestep: usize) -> f64 {
        let index = timestep % self.eff_cons.len();
        self.eff_cons[index]
    }

    pub fn get_cost_cons(&self, timestep: usize) -> f64 {
        let index = timestep % self.cost_cons.len();
        self.cost_cons[index]
    }
}

impl Serialize for Entity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Entity::Consumer(consumer) => consumer.serialize(serializer),
            Entity::Producer(producer) => producer.serialize(serializer),
            Entity::Storage(storage) => storage.serialize(serializer),
            Entity::Grid(grid) => grid.serialize(serializer),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Producer {
    pub entity_type: String,
    pub can_be_disabled: bool,
    pub power_prod: Vec<f64>,
    pub eff_prod: Vec<f64>,
    pub cost_prod: Vec<f64>,
    pub name: String,
    #[serde(skip_serializing)]
    pub produced_var: Vec<Variable>,

    pub produced: Vec<f64>,
}

impl Producer {
    pub fn new(
        power_prod: Vec<f64>,
        eff_prod: Vec<f64>,
        cost_prod: Vec<f64>,
        can_be_disabled: bool,

        name: String,
    ) -> Self {
        check_eff_vec(&eff_prod);
        Producer {
            power_prod,
            eff_prod,
            cost_prod,
            name,
            can_be_disabled,
            produced_var: vec![],
            produced: vec![],
            entity_type: "Producer".to_string(),
        }
    }

    pub fn get_power_prod(&self, timestep: usize) -> f64 {
        let index = timestep % self.power_prod.len();
        self.power_prod[index]
    }

    pub fn get_eff_prod(&self, timestep: usize) -> f64 {
        let index = timestep % self.eff_prod.len();
        self.eff_prod[index]
    }

    pub fn get_cost_prod(&self, timestep: usize) -> f64 {
        let index = timestep % self.cost_prod.len();
        self.cost_prod[index]
    }
}
#[derive(Debug, Serialize)]
pub struct Storage {
    pub power_prod: Vec<f64>,
    pub eff_prod: Vec<f64>,
    pub cost_prod: Vec<f64>,

    pub power_cons: Vec<f64>,
    pub eff_cons: Vec<f64>,
    pub cost_cons: Vec<f64>,

    pub storage_capacity: f64,
    pub start_capacity: f64,

    pub name: String,

    #[serde(skip_serializing)]
    pub produced_var: Vec<Variable>,
    #[serde(skip_serializing)]
    pub consumed_var: Vec<Variable>,

    pub produced: Vec<f64>,
    pub consumed: Vec<f64>,
    pub stored: Vec<f64>,

    pub entity_type: String,
}

impl Storage {
    pub fn new(
        cost_cons: Vec<f64>,
        eff_cons: Vec<f64>,
        power_cons: Vec<f64>,

        cost_prod: Vec<f64>,
        eff_prod: Vec<f64>,
        power_prod: Vec<f64>,

        storage_capacity: f64,
        start_capacity: f64,

        name: String,
    ) -> Self {
        check_eff_vec(&eff_prod);
        check_eff_vec(&eff_cons);
        Storage {
            cost_cons,
            eff_cons,
            power_cons,

            cost_prod,
            eff_prod,
            power_prod,

            storage_capacity,
            start_capacity,
            name,

            produced_var: vec![],
            consumed_var: vec![],
            produced: vec![],
            consumed: vec![],
            stored: vec![],
            entity_type: "Storage".to_string(),
        }
    }

    pub fn get_power_prod(&self, timestep: usize) -> f64 {
        let index = timestep % self.power_prod.len();
        self.power_prod[index]
    }

    pub fn get_eff_prod(&self, timestep: usize) -> f64 {
        let index = timestep % self.eff_prod.len();
        self.eff_prod[index]
    }

    pub fn get_cost_prod(&self, timestep: usize) -> f64 {
        let index = timestep % self.cost_prod.len();
        self.cost_prod[index]
    }

    pub fn get_power_cons(&self, timestep: usize) -> f64 {
        let index = timestep % self.power_cons.len();
        self.power_cons[index]
    }

    pub fn get_eff_cons(&self, timestep: usize) -> f64 {
        let index = timestep % self.eff_cons.len();
        self.eff_cons[index]
    }

    pub fn get_cost_cons(&self, timestep: usize) -> f64 {
        let index = timestep % self.cost_cons.len();
        self.cost_cons[index]
    }
}

#[derive(Debug, Serialize)]
pub struct Grid {
    pub name: String,

    pub power_prod: Vec<f64>,
    pub cost_prod: Vec<f64>,

    pub power_cons: Vec<f64>,
    pub cost_cons: Vec<f64>,


    #[serde(skip_serializing)]
    pub produced_var: Vec<Variable>,
    #[serde(skip_serializing)]
    pub consumed_var: Vec<Variable>,

    pub produced: Vec<f64>,
    pub consumed: Vec<f64>,

    pub entity_type: String,
}

impl Grid {
    pub fn new(
        cost_cons: Vec<f64>,
        power_cons: Vec<f64>,

        cost_prod: Vec<f64>,
        power_prod: Vec<f64>,

        name: String,
    ) -> Self {
        Grid {
            name,
            cost_prod,
            power_prod,
            cost_cons,
            power_cons,

            produced_var: vec![],
            consumed_var: vec![],
            produced: vec![],
            consumed: vec![],
            entity_type: "Grid".to_string(),
        }
    }

    pub fn get_cost_prod(&self, timestep: usize) -> f64 {
        let index = timestep % self.cost_prod.len();
        self.cost_prod[index]
    }

    pub fn get_cost_cons(&self, timestep: usize) -> f64 {
        let index = timestep % self.cost_cons.len();
        self.cost_cons[index]
    }

    pub fn get_power_prod(&self, timestep: usize) -> f64 {
        let index = timestep % self.power_prod.len();
        self.power_prod[index]
    }

    pub fn get_power_cons(&self, timestep: usize) -> f64 {
        let index = timestep % self.power_cons.len();
        self.power_cons[index]
    }
}

#[derive(Debug)]
pub enum Entity {
    Consumer(Consumer),
    Producer(Producer),
    Storage(Storage),
    Grid(Grid),
}
