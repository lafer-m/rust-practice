use std::thread;
use std::time::Duration;

// simulate a expensive function, speed about 2 seconds
pub fn simulated_expensive_calculation(intensity: u32)-> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2)); // sleep 2 seconds
    intensity
}


pub fn generate_wrokerout(intensity: u32, random: u32) {
    let expensive = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity > 25 {
        println!("do once {}", expensive(intensity));
        println!("do twice {}", expensive(intensity));
    } else {
        if random == 3 {
            println!("break for one day");
        } else {
            println!("only do once {}", expensive(intensity));
        }
    }
}


pub fn search<'a>(query: &str,contents: &'a str) -> Vec<&'a str> {
    let mut result = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}