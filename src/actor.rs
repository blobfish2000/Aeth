pub mod actor {

    pub enum Race {
        Human,
        Elf,
    }

    pub struct Actor<'a> {
        pub next_update: u32,
        pub granularity: u32,
        pub name: String,
        pub quantity: u32,
        pub age: u32,
        pub race: Race,
        pub job: String,
        pub dependants: Vec<&'a Actor<'a>>,
    }
}
