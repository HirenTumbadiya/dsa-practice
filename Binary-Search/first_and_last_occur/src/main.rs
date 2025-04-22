
pub fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() as isize - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid as usize] < target {
            left = mid + 1;
        } else if arr[mid as usize] > target {
            right = mid - 1;
        } else {
            if mid == 0 || arr[mid as usize - 1] != target {
                return Some(mid as usize);
            }
            right = mid - 1;
        }
    }
    None
}
pub fn last_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() as isize - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid as usize] < target {
            left = mid + 1;
        } else if arr[mid as usize] > target {
            right = mid - 1;
        } else {
            if mid == arr.len() as isize - 1 || arr[mid as usize + 1] != target {
                return Some(mid as usize);
            }
            left = mid + 1;
        }
    }
    None
}
fn main() {
    let arr = [1, 2, 2, 2, 3, 4, 5];
    let target = 11;

    if let Some(first_index) = first_occurrence(&arr, target) {
        println!("First occurrence of {} is at index {}", target, first_index);

        if let Some(last_index) = last_occurrence(&arr, target) {
            println!("Last occurrence of {} is at index {}", target, last_index);
        }
    } else {
        println!("{} not found", target);
    }
}
