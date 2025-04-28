pub fn find_nth_root(n: i32, k: i32) -> i32 {
    if n < 2 {
        return n;
    }
    let mut low = 1;
    let mut high = n / 2;

    while low <= high {
        let mid = low + (high - low) / 2;
        let power = (mid as i64).pow(k as u32);

        if power == n as i64 {
            return mid;
        } else if power < n as i64 {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    return -1 as i32;
}

fn main() {
    let n: i32 = 27;
    let k: i32 = 3;
    let result = find_nth_root(n, k);
    println!("The {}th root of {} is {}", k, n, result);
}
