struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

fn new_city(residents: u64, is_coastal: bool) -> City {
    let city_description = if is_coastal {
      "coastal"
    } else {
      "non-coastal"
    };
    let updated_description = format!("a *{}* city of approximately {} residents", city_description, residents);


    City {
        description: updated_description,
        residents,
        is_coastal,
    }
}

fn main() {
    let rustville: City = new_city(123_321, true);

    println!("This city can be described as: {}", rustville.description);

    if rustville.is_coastal {
        println!("It is a coastal city.");
    } else {
        println!("It is not a coastal city.");
    }
}
