enum CitySize {
    Town,       // approximate residents: 1_000
    City,       // approximate residents: 10_000
    Metropolis, // approximate residents: 1_000_000
}

struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

impl City {
    fn to_string(city: CitySize) -> String {
        (
            match city {
                CitySize::Town => {
                    "town"
                }

                CitySize::City => {
                    "city"
                }

                CitySize::Metropolis => {
                    "metropolis"
                }
            }
        ).to_string()
    }

    fn new(city_size: CitySize, is_coastal: bool) -> City {
        let residents = match city_size {
            CitySize::Town => {
                1_000
            }

            CitySize::City => {
                10_000
            }

            CitySize::Metropolis => {
                1000_000
            }
        };

        City {
            description: format!("a *{}* of approximately {} residents", City::to_string(city_size), residents),
            residents,
            is_coastal,
        }
    }
}

fn main() {
    let rustville = City::new(CitySize::Metropolis, true);

    println!("This city is {}", rustville.description);

    if rustville.residents > 100_000 {
        println!("Wow!");
    }
}
