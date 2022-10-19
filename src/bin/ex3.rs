use std::collections::HashMap;

use RustCourse2022::{City, CityData, load_cities};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cities: Vec<City> = load_cities()?;

    let mut countries: HashMap<String, Vec<City>> = HashMap::new();
    for city in cities {
        let name: &str = city.country_name_eng();

        /*
        if !countries.contains_key(name) {
            countries.insert(k: name.to_string(), v: vec![]);
        }

        let mut country_cities: &mut Vec<&City> = countries.get_mut(name).unwrap();
        country_cities.push(city);
         */

         countries.entry(name.to_string())
         .or_default()
         .push(city);
    }

    for city_list in countries.values_mut(){
        city_list.sort_by_key(|city| city.fields.population);
        // city_list.sort_by_key(|city: &&City| -city.fields.population); // reverse order
    }

    for (country_name, city_list) in &countries {
        println!("{}", country_name);

        for city in city_list {
            println!("\t{} {}", city.fields.name, city.fields.population)
        }
    }

    Ok(())
} 