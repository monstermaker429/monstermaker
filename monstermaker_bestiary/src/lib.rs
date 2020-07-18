use monstermaker_core::{Id, Name};
use std::collections::HashMap;
use std::marker::PhantomData;

pub trait BestiarySpeciesData<T, U, V> where
    T: Name,
    U: Name,
    V: Name {
    fn category(&self) -> &str;
    fn description(&self) -> &str;
    fn weight_in_hectograms(&self) -> u16;
    fn height_in_decimeters(&self) -> u16;
    fn color(&self) -> &T;
    fn shape(&self) -> &U;
    fn habitat(&self) -> &V;
}

pub trait Species<T, U, V>: BestiarySpeciesData<T, U, V> + Id + Name where
    T: Name,
    U: Name,
    V: Name {}

pub enum Status {
    None,
    Seen,
    Owned,
}

pub struct SeenEntry<'a> {
    pub name: &'a str,
}

pub struct OwnedEntry<'a> {
    pub category: &'a str,
    pub description: &'a str,
    pub weight_in_hectograms: u16,
    pub height_in_decimeters: u16,
}

pub struct Entry<'a> {
    pub status: &'a Status,
    pub seen_entry: Option<SeenEntry<'a>>,
    pub owned_entry: Option<OwnedEntry<'a>>,
}

pub struct Bestiary<'a, S, T, U, V> where 
    S: BestiarySpeciesData<T, U, V> + Id + Name, 
    T: Name, 
    U: Name, 
    V: Name {
    species: HashMap<u32, &'a S>,
    species_statuses: HashMap<u32, Status>,
    // PhantomData to reconcile type parameters that are unused in this
    // struct.
    phantom_t: PhantomData<T>,
    phantom_u: PhantomData<U>,
    phantom_v: PhantomData<V>,
}

impl <'a, S, T, U, V> Bestiary<'a, S, T, U, V> where 
    S: BestiarySpeciesData<T, U, V> + Id + Name, 
    T: Name, 
    U: Name, 
    V: Name {
    pub fn new(species_list: Vec<&'a S>) -> Bestiary<'a, S, T, U, V> {
        let mut species_map = HashMap::new();
        let mut species_statuses_map = HashMap::new();
        for species in species_list {
            species_map.insert(species.id(), species);
            species_statuses_map.insert(species.id(), Status::None);
        }
        Bestiary {
            species: species_map,
            species_statuses: species_statuses_map,
            phantom_t: PhantomData,
            phantom_u: PhantomData,
            phantom_v: PhantomData,
        }
    }
    
    pub fn view_species(&self, id: &u32) -> Option<Entry> {
        let status = self.species_statuses.get(id).unwrap_or(&Status::None);
        let species_or = self.species.get(id);
        match (status, species_or) {
            (_, None) => None,
            (Status::None, Some(_species)) => Some(Entry {
                status: status,
                seen_entry: None,
                owned_entry: None,
            }),
            (Status::Seen, Some(species)) => Some(Entry {
                status: status,
                seen_entry: Some(SeenEntry {
                    name: species.name(),
                }),
                owned_entry: None
            }),
            (Status::Owned, Some(species)) => Some(Entry {
                status: status,
                seen_entry: Some(SeenEntry {
                    name: species.name(),
                }),
                owned_entry: Some(OwnedEntry {
                    category: species.category(),
                    description: species.description(),
                    weight_in_hectograms: species.weight_in_hectograms(),
                    height_in_decimeters: species.height_in_decimeters(),
                }),
            }),
        }
    }
    
    pub fn see(&mut self, species: &(impl BestiarySpeciesData<T, U, V> + Id + Name)) where
        T: Name,
        U: Name,
        V: Name {
        let id = &species.id();
        match self.species_statuses.get(id).unwrap_or(&Status::Owned) {
            Status::None => {
                self.species_statuses.insert(*id, Status::Seen);
            },
            _ => {},
        }
    }
    
    pub fn own(&mut self, species: &(impl BestiarySpeciesData<T, U, V> + Id + Name)) where
        T: Name,
        U: Name,
        V: Name {
        let id = &species.id();
        match self.species_statuses.get(id).unwrap_or(&Status::Owned) {
            Status::Owned => {},
            _ => {
                self.species_statuses.insert(*id, Status::Owned);
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Bestiary, BestiarySpeciesData};
    use monstermaker_core::{Id, Name};
    
    struct Color {}
    struct Shape {}
    struct Habitat {}
    
    impl Name for Color {
        fn name(&self) -> &str {
            "color"
        }
    }
    impl Name for Shape {
        fn name(&self) -> &str {
            "shape"
        }
    }
    impl Name for Habitat {
        fn name(&self) -> &str {
            "habitat"
        }
    }
    
    struct Species {}
    
    impl Id for Species {
        fn id(&self) -> u32 {
            
        }
    }
    impl Name for Species {
        fn name(&self) -> &str {
            "species"
        }
    }
    
    #[test]
    fn foo() {
        struct Dummy {}
        
        impl Name for Dummy {
            fn name(&self) -> &str {
                "hello world"
            }
        }
        
        struct Species {}
        
        impl Id for Species {
            fn id(&self) -> u32 {
                0
            }
        }
        
        impl Name for Species {
            fn name(&self) -> &str {
                "foo"
            }
        }
        
        impl BestiarySpeciesData<Dummy, Dummy, Dummy> for Species {
            fn category(&self) -> &str { "bar"}
            fn description(&self) -> &str {"baz"}
            fn height_in_decimeters(&self) -> u16 {1}
            fn weight_in_hectograms(&self) -> u16 {2}
            fn color(&self) -> &Dummy {&Dummy{}}
            fn shape(&self) -> &Dummy {&Dummy {}}
            fn habitat(&self) -> &Dummy {&Dummy{}}
        }
        
        let b = Bestiary::new(vec![&Species{}]);
    }
}
