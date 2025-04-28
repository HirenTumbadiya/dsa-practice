

pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    if bloom_day.len() < (m * k) as usize {
        return -1;
    }

    let mut low = 1;
    let mut high = *bloom_day.iter().max().unwrap();
    let mut ans = high;

    while low <= high {
        let mid = (low + high) / 2;
        let mut total = 0;
        let mut count = 0;

        for &day in &bloom_day {
            if day <= mid {
                count += 1;
                if count == k {
                    total += 1;
                    count = 0;
                }
            } else {
                count = 0;
            }
        }

        if total >= m {
            ans = mid;
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    ans
}


fn main() {
    let bloom_day = vec![1, 10, 3, 10, 2];
    let m = 3;
    let k = 1;
    let result = min_days(bloom_day, m, k);
    println!("Minimum days: {}", result);
}
