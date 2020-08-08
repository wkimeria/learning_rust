use std::collections::HashMap;

fn main() {

    // Given a list of integers, use a vector and return the mean (the average value), 
    // median (when sorted, the value in the middle position), 
    // and mode (the value that occurs most often); 
    // a hash map will be helpful here) of the list.

    let v :Vec<f32> = vec![1.0,1.0,1.0,53.0,5.0,5.5,7.0, 5.5];

    let mean = get_mean(&v);
    println!("Mean = {}", mean);

    let median = get_median(&v);

    println!("Median = {}", median);

    let mode = get_mode(&v);

    println!("Mode = {}", mode);


}

// Get mean (average value) of vector passed in
fn get_mean(lst: &Vec<f32>) -> f32 {

    let mut total :f32 = 0.0;

    for i in lst.iter() {
        total = total + i;
    }

    return ( total / lst.len() as f32) as f32;
}

// get median (when sorted, the value in the middle position), 
fn get_median(lst: &Vec<f32>) -> f32 {

        let mut lst_sort: Vec<f32> = lst.clone();

        //Sort the vector
        lst_sort.sort_by(|a, b| a.partial_cmp(b).unwrap());

        //Get the midpoint
        let mid_point :usize = lst.len() / 2;

        //Return midpoint

        return match lst.get(mid_point){
            Some(idx) => *idx,
            None => 0.0, 
        }
}

//get mode (the value that occurs most often); 
fn get_mode(lst: &Vec<f32>) -> f32 {

    // Put counts into collection
    let mut counts : HashMap<String,i32> = HashMap::new();

    for i in lst.iter() {
        // String conversion because f32 cannot be Map Key
        let count = counts.entry(i.to_string()).or_insert(0);
        *count += 1;
    }

    let most_common = counts.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    
    // Get return and convert back to f32
    return match most_common.0.parse::<f32>() {
        Ok(i) => i,
        Err(_) => 0.0
    }
}

