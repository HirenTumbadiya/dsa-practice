pub fn floor(arr: &Vec<i32>, target: i32) -> i32 {
    let n = arr.len();
    if n == 0 {
        return -1;
    }
    let mut low = 0;
    let mut high = n - 1;
    let mut ans: i32 = -1;
    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] <= target{
            ans = mid as i32;
            if mid == n - 1 { break; }
            low = mid + 1;
        }else{  
            high = mid - 1;
        }
    }
    ans as i32
}

pub fn ceil(arr: &Vec<i32>, target: i32) -> i32 {
    let n = arr.len();
    if n == 0 {
        return -1;
    }
    let mut low = 0;
    let mut high = n - 1;
    let mut ans: i32 = -1;
    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] >= target{
            ans = mid as i32;
            if mid == 0 { break; }
            high = mid - 1;
        }else{  
            low = mid + 1;
        }
    }
    ans as i32
}

pub fn floor_ceil(arr: &Vec<i32>, target: i32) -> (i32, i32) {
    let floor_val = floor(arr, target);
    let ceil_val = ceil(arr, target);
    (floor_val, ceil_val)
}

fn main() {
    let arr = vec![1, 3, 8, 10, 12, 15, 18, 20];
    let target = 15;

    let (floor_val, ceil_val) = floor_ceil(&arr, target);
    println!("Floor Value: {}, Ceiling Value: {}", floor_val, ceil_val);
}
