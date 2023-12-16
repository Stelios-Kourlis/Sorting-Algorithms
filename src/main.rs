mod algorithms;

use std::process::exit;
use rand::Rng;

fn main() {
    let mut array :Vec<i32> = vec![]; 
    let mut get_array_check = true;
    let mut result_times: Vec<String> = vec![];
    println!("Welcome to Sorting Algorithms Bechmark");
    loop{
        if get_array_check{
            array = get_array();
            get_array_check = false;
            println!("Your array is: {:?}", array);
        }
        let choice = get_valid_algorithm_choice();
        //apply_algorithm(choice, &array);
        match choice {
            -2 => print_result_times(&result_times),
            -1 => get_array_check = true,
            0 => exit(0),
            1..=algorithms::NUMBER_OF_ALGORITHMS => result_times.push(apply_algorithm(choice, &array)),
            _ => println!("Something Went wrong"),
        };
    }; 
}

fn get_array() -> Vec<i32>{
    let mut has_chosen_invalid_at_least_once = false;
    loop {
        if !has_chosen_invalid_at_least_once { println!("To start first say if you want:"); }
        else {println!("Not a valid choice, please choose again: ")}
        println!("(1) The hard coded list of 100 numbers\n(2) A random list");
        let array_choice = match get_i32_from_user() {
            Some(choice) => choice,
            None => -1,
        };
        match create_array(array_choice) {
            Some(array) => return array,
            None => {},
        };
        has_chosen_invalid_at_least_once = true;
    }
}

fn get_valid_algorithm_choice() -> i32 {
    loop{
        println!("---------------------");
        println!("Choose the algorithm:");
        println!("(2) Selection Sort");
        println!("(1) Bubble Sort");
        println!("---------------------");
        println!("(0) Exit");
        println!("(-1) Change Array");
        println!("(-2) See Previous Times");
        println!("---------------------");
        match get_i32_from_user() {
            Some(num) => {
                if num < -2 { }
                else if num > algorithms::NUMBER_OF_ALGORITHMS { }
                else { return num }
            },
            None => {},
        };
    } 
}

fn print_result_times(times : &Vec<String>){
    println!("Current Sort Times:\n");
    for time in times{ println!("{}", time); }
}

fn apply_algorithm(algorithm : i32, array : &Vec<i32>) -> String{
    match algorithm {
        2 => algorithms::selection_sort(array),
        1 => algorithms::bubble_sort(array),
        _ => { println!("This shouldnt have happened"); String::from("Something Went Wrong on this one") },
    }
}

fn get_i32_from_user() -> Option<i32>{
    let mut input : String = "".to_string();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(err) => { println!("Something Went Wrong: {}", err); return None; },
    }
    match input.trim().parse::<i32>(){
        Ok(num) => return Some(num),
        Err(err) => { println!("Something Went Wrong: {}", err); return None; },
    };
}

fn create_array(option : i32) -> Option<Vec<i32>>{
    match option {
        1 => return Some(vec![42406,55232,77387,41377,13090,17697,1542,59599,80110,89922,19425,6805,7775,41987,78683,88087,15198,53720,40190,50401,20992,12126,60234,57687,74432,69860,21408,92573,47061,69517,74243,29646,93156,59706,46304,99979,57202,32246,76459,71357,96016,86795,52497,65197,36059,14933,76150,96095,66114,49899,16456,88154,56977,81697,14596,69496,57764,70317,72283,36216,39281,40678,15650,69611,92151,155,91750,98736,27225,62529,98573,75156,70289,53588,34121,83011,5925,65072,13324,2140,56594,57964,32659,76666,49331,92045,27602,26780,9320,55634,11169,45201,95280,16544,23408,48762,57737,37373,62331,96956]),
        2 => return generate_random_array(),
        _ => return None,
    };
}

fn generate_random_array() -> Option<Vec<i32>>{
    let size : i32 = loop {
        println!("Type Array Size (0 for a set size of 100): ");
        match get_i32_from_user() {
            Some(num) => {
                if num < 0 { }
                else if num == 0  { break 100 }
                else { break num }
            },
            None => continue,
        };
    };
    let lower_bound : i32 = loop {
        println!("Choose Lower Range Bound (Minimum): ");
        match get_i32_from_user() {
            Some(bound) => break bound,
            None => continue,
        };

    };
    let upper_bound : i32 = loop{
        println!("Choose Upper Range Bound: ");
        match get_i32_from_user() {
            Some(bound) => {
                if bound > lower_bound { break bound }
                else {}
            },
            None => continue,
        };
    };
    let mut vec : Vec<i32> = vec![];
    for _ in 0..size{
        let num : i32 = match rand::thread_rng().gen_range(lower_bound..=upper_bound).try_into(){
            Ok(value) =>  value,
            Err(_) => return None,
        };
        vec.push(num);
    }
    return Some(vec);
}
