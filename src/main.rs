use std::io::{self, Write};
use crossterm::{
    cursor,
    execute,
    queue,
    event::{self, KeyCode, KeyEvent },
    terminal::{self, ClearType},
};

struct Planet {
    name: &'static str,
    description: &'static str,
    image: &'static str,
}

const PLANETS: [Planet; 9] = [
    Planet {
        name: "Mercury",
        description: "The smallest planet in our solar system.",
        image: "🪐",
    },
    Planet {
        name: "Venus",
        description: "A rocky planet with a thick atmosphere.",
        image: "🪐",
    },
    Planet {
        name: "Earth",
        description: "The planet we live on.",
        image: "🪐",
    },
    Planet {
        name: "Mars",
        description: "A cold and dry planet.",
        image: "🪐", 
    },
    Planet {
        name: "Jupiter",
        description: "A gas giant.",
        image: "🪐",
    },
    Planet {
        name: "Saturn",
        description: "A gas giant.",
        image: "🪐",
    },
    Planet {
        name: "Uranus",
        description: "A gas giant.",
        image: "🪐",
    },
    Planet {
        name: "Neptune",
        description: "A gas giant.",
        image: "🪐",
    },
    Planet {
        name: "Pluto",
        description: "A dwarf planet.",
        image: "🪐",
    }
];

fn list_planets() {
    for planet in PLANETS.iter() {
        println!("{}", planet.name);
    }
}

fn get_planet_info(planet_name: &str) {
    if let Some(planet) = PLANETS.iter().find(|&p| p.name == planet_name) {
        println!("Name: {}", planet.name);
        println!("Description: {}", planet.description);
    } else {
        println!("Planet not found: {}", planet_name);
    }
}

fn render_image(image: &str) {
    print!("{}", image);
}

fn main() {
    println!("Choose the number for the interaction: ");
    println!("1. List planets");
    println!("2. Get planet info");
    println!("3. Render image");
    println!("4. Quit");

    loop {
        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().parse::<u32>().unwrap();

        if input == 1 {
            list_planets();
        } else if input == 2 {
            println!("Enter planet name: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            get_planet_info(input);
        } else if input == 3 {
            println!("Enter planet name: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            render_image(input);
        } else if input == 4 {
            println!("Invalid input");
            return;
        }
    }


    // Wait for user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("\n");
}
