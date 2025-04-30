struct Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        if k < arr[0] {
            return k;
        }

        let mut low = 0;
        let mut high = arr.len();

        while low < high {
            let mid = low + (high - low) / 2;
            let missing = arr[mid] - (mid as i32 + 1);
            if missing < k {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        low as i32 + k   
    }
}

fn main() {
    let arr = vec![2, 3, 4, 7, 11];
    let k = 5;
    let result = Solution::find_kth_positive(arr, k);
    println!("The {}th missing positive number is: {}", k, result);
}
