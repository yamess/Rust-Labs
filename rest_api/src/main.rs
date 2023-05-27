use std::collections::HashMap;

fn main() {
    let nums_1 = vec![2,7,11,15];
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut dict: HashMap<i32, i32> = HashMap::new();
    let mut result: Vec<i32> = Vec::new();

    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if dict.contains_key(&complement) {
            result.push(*dict.get(&complement).unwrap());
            result.push(i as i32);
            return result;
        }
        dict.insert(*num, i as i32);
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2,7,11,15], 9), vec![0,1]);
        assert_eq!(two_sum(vec![3,2,4], 6), vec![1,2]);
        assert_eq!(two_sum(vec![3,3], 6), vec![0,1]);
    }
}