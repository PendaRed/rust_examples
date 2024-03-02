
#[derive(Debug, Clone, Copy)]
pub enum Reproduction {
    Egg,
    Born,
    Pounch
}

#[derive(Debug)]
pub struct Animal {
    species: String,
    legs: u32,
    wings: bool,
    reproduction: Reproduction
}

impl Animal {
    pub fn new(species: String,
        legs: u32,
        wings: bool,
        reproduction: Reproduction) -> Animal {
            Animal{species, legs, wings, reproduction}
        }
    pub fn with_leg_change(&self, no_legs:u32)->Animal {
        Animal{legs:no_legs, species: self.species.clone(), ..*self}
    }
    pub fn desc_repro(&self) -> String {
        let repro = match &self.reproduction  {
            Reproduction::Egg => "lays eggs",
            Reproduction::Born => "gives birth",
            Reproduction::Pounch => "is a marsupial",
        };
        format!("Animal {} Repro: {}", self.species, repro)
    }
}

