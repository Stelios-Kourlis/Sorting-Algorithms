pub const NUMBER_OF_ALGORITHMS : i32 = 2;

pub fn bubble_sort(array : &Vec<i32>) -> String{
    println!("--Starting Bubble Sort--\n...");
    let mut vec = array.clone();
    let start_time = std::time::SystemTime::now();
    for items_sorted in 0..=vec.len()-1{
        for index in 0..=vec.len()-items_sorted{
            if index == vec.len()-1 { break };
            if vec[index] > vec[index+1]{ 
                switch_numbers_at_indexes(&mut vec, index, index+1); 
            }
        }
    }
    let end_time = std::time::SystemTime::now();
    let duration = end_time.duration_since(start_time);
    return match duration {
        Ok(dur) => {println!("--Bubble Sort Done--\nYour new array is\n{:?}\nCalculated in {:?} ms", vec, dur.as_secs_f64()); format_result("Bubble Sort", array.len(), dur) },
        Err(err) => {print!("Something went wrong with the timing, [start_time: {:#?}, end_time: {:#?}] specifically {}", start_time, end_time, err); String::from("Bubble Sort Failed")},
    }
}

pub fn selection_sort(array : &Vec<i32>) -> String{
    println!("--Starting Selection Sort--\n...");
    let mut vec = array.clone();
    let start_time = std::time::SystemTime::now();
    //let start_index = 0;
    let mut smallest_num_index = 0;
    for start_index in 0..vec.len() {
        for index in start_index..vec.len() {
            if vec[index] < vec[smallest_num_index] {smallest_num_index = index;}
        }
        switch_numbers_at_indexes(&mut vec, start_index, smallest_num_index);
    }
    let end_time = std::time::SystemTime::now();
    let duration = end_time.duration_since(start_time);
    return match duration {
        Ok(dur) => {println!("--Selection Sort Done--\nYour new array is\n{:?}\nCalculated in {:?} ms", vec, dur.as_secs_f64()); format_result("Selection Sort", array.len(), dur) },
        Err(err) => {print!("Something went wrong with the timing, [start_time: {:#?}, end_time: {:#?}] specifically {}", start_time, end_time, err); String::from("Selection Sort Failed")},
    }
}

fn format_result(algorithm_name: &str, array_size: usize, duration: std::time::Duration) -> String{
    format!("{} on {} elements took {} ms", algorithm_name, array_size, duration.as_secs_f64())
}

fn switch_numbers_at_indexes(array: &mut Vec<i32>, index1: usize, index2: usize){
    let num = array[index1];
    array[index1] = array[index2];
    array[index2] = num;  
}