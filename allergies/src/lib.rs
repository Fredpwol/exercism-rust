use std::{collections::HashMap, hash::Hash};

pub struct Allergies {
    score: u32
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergen{

    fn get_allegies_score() -> HashMap<Self, u32>{
        let mut allegies_score: HashMap<Allergen, u32> = HashMap::new();
        use Allergen::*;
        allegies_score.insert(Eggs, 1);
        allegies_score.insert(Peanuts, 2);
        allegies_score.insert(Shellfish, 4);
        allegies_score.insert(Strawberries, 8);
        allegies_score.insert(Tomatoes, 16);
        allegies_score.insert(Chocolate, 32);
        allegies_score.insert(Pollen, 64);
        allegies_score.insert(Cats, 128);
        return allegies_score;
    }

    fn get_score(&self) -> u32{
        *Allergen::get_allegies_score().get(&self).unwrap_or(&0)
    }

}


impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergy_score = allergen.get_score();
        (self.score & allergy_score) == allergy_score
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergen_list: Vec<Allergen> = Vec::new();
        for (allergen, _) in Allergen::get_allegies_score(){
            if self.is_allergic_to(&allergen){
                allergen_list.push(allergen.clone())
            }
        }
        return allergen_list;

    }
}
