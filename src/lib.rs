use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct RestrictionEnzymeChardata {
    // (3, -3, None, None, "TTATAA"),
    start: i32,
    stop: i32,
    something1: Option<String>,
    something2: Option<String>,
    site: String,
}

// Define the People struct to hold first and last names
#[derive(Debug, Serialize, Deserialize)]
struct RestrictionEnzyme {
    name: String,
    charac: RestrictionEnzymeChardata,
    compsite: String,
    dna: Option<String>,
    freq: f32,
    fst3: i32,
    fst5: i32,
    id: i32,
    inact_temp: i32,
    opt_temp: i32,
    ovhg: i32,
    ovhgse: Option<String>,
    results: Option<String>,
    scd3: Option<String>,
    scd5: Option<String>,
    site: String,
    size: i32,
    substrat: String,
    suppl: Vec<String>,
    uri: String,
}

// Define the People struct to hold first and last names
#[derive(Debug, Serialize, Deserialize)]
struct People {
    first_name: String,
    last_name: String,
}

// Define an enum to represent different individuals
enum Person {
    Alice,
    Bob,
}

// Create a function to load data for each person
fn load_person_data(person: Person) -> People {
    let filename = match person {
        Person::Alice => "data/people/alice.json",
        Person::Bob => "data/people/bob.json",
        // Add more cases as needed
    };

    // Read and deserialize data from JSON file
    let json_data = fs::read_to_string(filename).expect("Failed to read JSON file");
    serde_json::from_str(&json_data).expect("Failed to deserialize JSON data")
}

enum RestrictionEnzymeEnum { 
    Aani,
   Aari,
   Aasi,
   Aatii,
}


fn load_restrictionenzyme_data(enzyme: RestrictionEnzymeEnum) -> RestrictionEnzyme {
    let filename = match enzyme {
       RestrictionEnzymeEnum::Aani => "data/restriction_enzymes/enzymedata/Aani.json",
      RestrictionEnzymeEnum::Aari => "data/restriction_enzymes/enzymedata/Aari.json",
      RestrictionEnzymeEnum::Aasi => "data/restriction_enzymes/enzymedata/Aasi.json",
      RestrictionEnzymeEnum::Aatii => "data/restriction_enzymes/enzymedata/Aatii.json",
       };
   let json_data = fs::read_to_string(filename).expect("Failed to read JSON file");
   let redata = serde_json::from_str(&json_data).expect("Failed to deserialize JSON data");
   redata
}



fn main() {
    let alice = load_person_data(Person::Alice);
    println!("Alice! {:?}", alice);
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let aari = load_restrictionenzyme_data(RestrictionEnzymeEnum::Aari);
        println!("aari! {:?}", aari);

        let alice = load_person_data(Person::Alice);
        println!("Alice! {:?}", alice);

        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
