// Exercise: Write a program that prints the largest city for each US state
// (States are in admin1_code)

use std::collections::HashMap;

use RustCourse2022::{City, load_cities};

/*
fn get_population(city: &&City) -> i64 {
    return -city.fields.population;
}
 */

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cities: Vec<City> = load_cities()?;

    let mut cities_by_state: HashMap<String, Vec<&City>> = HashMap::new();
    for city in &cities {
        if &city.fields.country_code != "US" {
            continue;
        }

        let state = match &city.fields.admin1_code {
            Some(x) => x.to_string(),
            None => continue,
        };

        cities_by_state.entry(state)
        .or_default()
        .push(city);       
    }

    for cities in cities_by_state.values_mut() {
        cities.sort_by_key(|city| city.fields.population);

        // Other option using function
        //cities.sort_by_key(get_population); // sorts in reverse order
    }

    for (state, cities) in cities_by_state {
        println!("{} {}", state, cities[0].fields.name);
    }
    
    Ok(())
} 