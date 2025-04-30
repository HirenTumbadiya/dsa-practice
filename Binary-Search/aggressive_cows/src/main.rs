pub fn aggressive_cows(stalls: Vec<i32>, k: i32) -> i32 {
    let mut stalls = stalls;
    stalls.sort();
    let (mut low, mut high) = (1, stalls.last().unwrap() - stalls.first().unwrap());
    let mut ans = 0;

    while low <= high {
        let mid = (low + high) / 2;
        if can_place_cows(&stalls, mid, k) {
            ans = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    ans
}
fn can_place_cows(stalls: &[i32], min_dist: i32, k: i32) -> bool {
    let mut count = 1;
    let mut last_pos = stalls[0];

    for &stall in &stalls[1..] {
        if stall - last_pos >= min_dist {
            count += 1;
            last_pos = stall;
        }
        if count >= k {
            return true;
        }
    }
    false
}

fn main() {
    let stalls = vec![1, 2, 4, 8, 9];
    let k = 3;
    let result = aggressive_cows(stalls, k);
    println!("The largest minimum distance is: {}", result);
}
