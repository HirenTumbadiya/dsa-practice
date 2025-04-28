fn find_max(piles: &Vec<i32>) -> i32 {
    let mut max = piles[0];
    for &pile in piles.iter().skip(1) {
        if pile > max {
            max = pile;
        }
    }
    max
}

pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    if piles.is_empty() {
        return -1;
    }

    let mut low = 1;
    let mut high = find_max(&piles);
    let mut ans = high;

    while low <= high {
        let mid = (low + high) / 2;
        let mut total: i64 = 0;

        for &pile in &piles {
            total += (pile as i64 + mid as i64 - 1) / mid as i64;
        }

        if total <= h as i64 {
            ans = mid;
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    ans
}

fn main() {
    let piles = vec![805306368, 805306368, 805306368];
    let h = 1000000000;
    let result = min_eating_speed(piles, h);
    println!("Minimum eating speed: {}", result);
}
