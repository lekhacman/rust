fn main() {
    let collection = vec![1, 6, 5 , 1];

    assert_eq!(stats::mean(&collection), 3.25);
    assert_eq!(stats::mean(&vec![2,4]), 3.0);

    assert_eq!(stats::mode(&collection), vec![1]);

    let collection2 = vec![2,3,2,5,7,8,7];
    assert_eq!(stats::mode(&collection), vec![1]);

    let mut modes2 = stats::mode(&collection2);
    modes2.sort();
    assert_eq!(modes2, vec![2, 7]);

    assert_eq!(stats::median(&vec![1,2,3]), 2.0);
    assert_eq!(stats::median(&vec![1]), 1.0);
    assert_eq!(stats::median(&vec![2, 4, 3, 5]), 3.5);
    assert_eq!(stats::median(&collection), 3.0);
    assert_eq!(stats::median(&collection2), 5.0);
}

/**
Given a list of integers, use a vector
and return the mean (the average value),
median (when sorted, the value in the middle position),
and mode (the value that occurs most often;
a hash map will be helpful here) of the list.
*/
mod stats {
    use std::collections::HashMap;
    use std::prelude::v1::Vec;

    pub fn mean(collection: &Vec<i32>) -> f64 {
        let sum: i32 = collection.iter().sum();
        f64::from(sum) / (collection.len() as f64)
    }

    pub fn mode(collection: &Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for val in collection {
            let count = map.entry(*val).or_insert(0);
            *count += 1;
        }
        let mut result = Vec::new();
        let mut roof = 0;
        for (key, value) in map.iter() {
            if *value > roof {
                roof = *value;
                result.clear();
                result.push(*key);
            } else if *value == roof {
                result.push(*key);
            }
        }
        result
    }

    pub fn median(collection: &Vec<i32>) -> f64 {
        let mut sorted = collection.clone();
        sorted.sort();
        let len = sorted.len();

        let result = if len % 2 != 0 {
            let n = (len + 1) / 2;
            let value = &sorted[(n - 1)];
            f64::from(*value)

        } else {
            let n = len / 2;
            mean(&vec![sorted[n-1], sorted[n]])
        };

        result
    }
}