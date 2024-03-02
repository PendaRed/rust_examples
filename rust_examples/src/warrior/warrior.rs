

pub enum Gender {
    Male, Female
}

pub enum InjurySeverity {
    Minor, Major, Critical
}
pub enum Injury {
    Head(InjurySeverity),
    LeftArm(InjurySeverity),
    RightArm(InjurySeverity),
    LeftLeg(InjurySeverity),
    RightLeg(InjurySeverity),
    Torso(InjurySeverity),
}

pub struct Warrior {
    pub name:String,
    pub gender: Gender,
    pub age: u32,
    pub injuries: Vec<Injury>
}

impl Warrior {
    fn new(name:&str, gender:Gender, age:u32) -> Warrior {
        Warrior{name:"Gimli".to_string(), gender, age, injuries :vec![]}
    }
}