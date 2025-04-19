pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    let mut low = 0;
    let mut high = n - 1;
    let mut ans = n;

    while low <= high {
        let mid = low + (high - low) / 2;
        if nums[mid] >= target {
            ans = mid;
            if mid == 0 { break; }
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    ans as i32
}

fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 5;
    let index = search_insert(nums, target);
    println!("Target index: {}", index); // Output should be 2
}             