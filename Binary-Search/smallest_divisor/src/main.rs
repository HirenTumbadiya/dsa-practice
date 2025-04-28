pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
    let mut left = 1;
    let mut right = match nums.iter().max() {
        Some(&val) => val,
        None => return 1,
    };

    while left < right {
        let mid = (left + right) / 2;
        let sum: i32 = nums.iter().map(|&x| (x + mid - 1) / mid).sum();

        if sum > threshold {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
    
}



fn main() {
    let nums = vec![1,2,5,9];
    let threshold = 6;
    let result = smallest_divisor(nums, threshold);
    println!("The smallest divisor is: {}", result);
}
