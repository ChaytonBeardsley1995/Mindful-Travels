mod travels {

    #[derive(Debug)]
    pub struct Travel {
        pub id: i32,
        pub country: String,
        pub description: String,
        pub duration: i32,
        pub cost: i32,
    }

    pub fn create_travel(id: i32, country: String, description: String, duration: i32, cost: i32) -> Travel {
        Travel {
            id,
            country,
            description,
            duration,
            cost,
        }
    }

    pub fn list_travels() {
        let travels = vec![
            create_travel(1, "France".to_string(), "Visit Eiffel Tower".to_string(), 4, 75_00),
            create_travel(2, "Spain".to_string(), "Sightseeing in Madrid".to_string(), 5, 85_00),
            create_travel(3, "Italy".to_string(), "Taste Italian Cuisine".to_string(), 7, 95_00),
            create_travel(4, "Greece".to_string(), "Explore Ancient Ruins".to_string(), 10, 100_00),
            create_travel(5, "India".to_string(), "Ride an Elephant".to_string(), 3, 70_00),
            create_travel(6, "Thailand".to_string(), "Discover the Floating Markets".to_string(), 8, 90_00),
            create_travel(7, "China".to_string(), "View the Great Wall".to_string(), 10, 100_00),
            create_travel(8, "Brazil".to_string(), "Experience the Carnaval".to_string(), 7, 95_00),
            create_travel(9, "South Africa".to_string(), "Climb Table Mountain".to_string(), 5, 85_00),
            create_travel(10, "Vietnam".to_string(), "Visit the Cu Chi Tunnels".to_string(), 6, 80_00),
        ];

        for t in travels {
            println!("Travel: {:?}", t);
        }
    }

    pub fn get_cheapest_travel() -> Travel {
        let travels = vec![
            create_travel(1, "France".to_string(), "Visit Eiffel Tower".to_string(), 4, 75_00),
            create_travel(2, "Spain".to_string(), "Sightseeing in Madrid".to_string(), 5, 85_00),
            create_travel(3, "Italy".to_string(), "Taste Italian Cuisine".to_string(), 7, 95_00),
            create_travel(4, "Greece".to_string(), "Explore Ancient Ruins".to_string(), 10, 100_00),
            create_travel(5, "India".to_string(), "Ride an Elephant".to_string(), 3, 70_00),
            create_travel(6, "Thailand".to_string(), "Discover the Floating Markets".to_string(), 8, 90_00),
            create_travel(7, "China".to_string(), "View the Great Wall".to_string(), 10, 100_00),
            create_travel(8, "Brazil".to_string(), "Experience the Carnaval".to_string(), 7, 95_00),
            create_travel(9, "South Africa".to_string(), "Climb Table Mountain".to_string(), 5, 85_00),
            create_travel(10, "Vietnam".to_string(), "Visit the Cu Chi Tunnels".to_string(), 6, 80_00),
        ];

        let mut cheapest_travel = travels[0];
        for t in travels {
            if t.cost < cheapest_travel.cost {
                cheapest_travel = t;
            }
        }

        cheapest_travel
    }

    pub fn book_travel(id: i32) -> Result<String, &'static str> {
        let travels = vec![
            create_travel(1, "France".to_string(), "Visit Eiffel Tower".to_string(), 4, 75_00),
            create_travel(2, "Spain".to_string(), "Sightseeing in Madrid".to_string(), 5, 85_00),
            create_travel(3, "Italy".to_string(), "Taste Italian Cuisine".to_string(), 7, 95_00),
            create_travel(4, "Greece".to_string(), "Explore Ancient Ruins".to_string(), 10, 100_00),
            create_travel(5, "India".to_string(), "Ride an Elephant".to_string(), 3, 70_00),
            create_travel(6, "Thailand".to_string(), "Discover the Floating Markets".to_string(), 8, 90_00),
            create_travel(7, "China".to_string(), "View the Great Wall".to_string(), 10, 100_00),
            create_travel(8, "Brazil".to_string(), "Experience the Carnaval".to_string(), 7, 95_00),
            create_travel(9, "South Africa".to_string(), "Climb Table Mountain".to_string(), 5, 85_00),
            create_travel(10, "Vietnam".to_string(), "Visit the Cu Chi Tunnels".to_string(), 6, 80_00),
        ];

        let found_travel = travels.iter().find(|t| t.id == id);

        if found_travel != None {
            let travel = found_travel.unwrap();
            return Ok(format!("You have booked the {} tour for ${}. Enjoy your trip!", travel.description, travel.cost));
        } else {
            return Err("Travel not found");
        }
    }

}

fn main() {
    travels::list_travels();

    let cheapest_travel = travels::get_cheapest_travel();
    println!("Cheapest Travel: {:?}", cheapest_travel);

    let result = travels::book_travel(2);

    match result {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error: {}", e),
    }
}