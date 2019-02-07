fn main() {
    let collection = vec![1, 6, 5 , 1];

    assert_eq!(stats::mean(&collection), 3.25);
    assert_eq!(stats::mean(&vec![2,4]), 3.0);

}

/**
Given a list of integers, use a vector
and return the mean (the average value),
median (when sorted, the value in the middle position),
and mode (the value that occurs most often;
a hash map will be helpful here) of the list.
*/
mod stats {
    pub fn mean(collection: &Vec<i32>) -> f64 {
        let mut sum = 0.0;
        for val in collection {
            sum += f64::from(*val);
        }

        sum / (collection.len() as f64)
    }
}