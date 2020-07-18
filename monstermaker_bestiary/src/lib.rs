use monstermaker_core::{Id, Name};
use std::collections::HashMap;

pub trait BestiarySpeciesData {
    fn category(&self) -> &str;
    fn description(&self) -> &str;
    fn weight_in_hectograms(&self) -> u16;
    fn height_in_decimeters(&self) -> u16;
    fn color<T: Name>(&self) -> &T;
    fn shape<T: Name>(&self) -> &T;
    fn habitat<T: Name>(&self) -> &T;
}

trait Species: BestiarySpeciesData + Id + Name {}

pub enum Status {
    None,
    Seen,
    Owned,
}

pub struct SeenEntry {
    pub name: &'static str,
}

pub struct OwnedEntry {
    pub category: &'static str,
    pub description: &'static str,
    pub weight_in_hectograms: u16,
    pub height_in_decimeters: u16,
}

pub struct Entry<'a> {
    pub status: &'a Status,
    pub seen_entry: Option<SeenEntry>,
    pub owned_entry: Option<OwnedEntry>,
}

pub struct Bestiary<S: impl Species> {
    species: HashMap<u16, &'static S>,
    species_statuses: HashMap<u16, Status>,
}

impl Bestiary<S: impl Species> {
    pub fn new(species_list: Vec<&'static Species>) -> Bestiary {
        let mut species_map = HashMap::new();
        let mut species_statuses_map = HashMap::new();
        for species in species_list {
            species_map.insert(species.id(), species);
            species_statuses_map.insert(species.id(), Status::None);
        }
        Bestiary {
            species: species_map,
            species_statuses: species_statuses_map,
        }
    }
    
    pub fn view_species(&self, id: &u16) -> Option<Entry> {
        let status = self.species_statuses.get(id).unwrap_or(&Status::None);
        match (status, self.species.get(id)) {
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
    
    pub fn see(&mut self, species: &impl Species) {
        let id = &species.id();
        match self.species_statuses.get(id).unwrap_or(&Status::Owned) {
            Status::None => {
                self.species_statuses.insert(*id, Status::Seen);
            },
            _ => {},
        }
    }
    
    pub fn own(&mut self, species: &impl Species) {
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
    static TYPE_A: Type = Type {
        
    }
    static SPECIES_A: Species = Species {
        
    }
    
    #[test]
    fn 
}
