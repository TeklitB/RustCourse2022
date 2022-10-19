
use RustCourse2022::{City, CityData};

//pub enum Result<T, E>{
//    Ok(T),
 //   Error(E),
//}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //println!("Hello, world!");
/* 
    let data: String = match std::fs::read_to_string(path:"cities100k.json"){
        Ok(x: String) => x,
        Error(e) => {
            eprintln!("{:?}", e);
            return Err(e);
        },
    };
*/
     let data: String = std::fs::read_to_string("cities100k.json")?;
    // do something with data
    let cities: Vec<City> = serde_json::from_str(&data)?;

    for city in &cities {
        if city.fields.country_code != "US" {
            continue;
        }

        let state = match &city.fields.admin1_code {
            Some(x) => x,
            None => {
                continue;
            },
        };

        if state != "CA" {
            continue;
        }

        if city.fields.population < 1000000 {
            continue;
        }

        println!("{:#?}", city)
    }

    Ok(())
}
