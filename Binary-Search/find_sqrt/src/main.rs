pub fn find_sqrt(x: i32) -> i32 {
    if x < 2 {
        return x as i32;
    }
    let mut low = 1;
    let mut high = x / 2;

    while low <= high {
        let mid = low + (high - low) / 2;
        let square = mid * mid;

        if square == x {
            return mid as i32;
        } else if square < x {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    return high as i32;
}


fn main() {
    let x:i32 = 36;
    let result = find_sqrt(x);
    println!("The square root of {} is {}", x, result);
}
