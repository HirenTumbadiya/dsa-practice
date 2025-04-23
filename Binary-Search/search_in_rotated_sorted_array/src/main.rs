pub fn unique_search(nums: Vec<i32>, target: i32) -> i32{
    let n = nums.len();
    if n == 0 {
        return -1;
    }
    let mut low = 0;
    let mut high = n - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        if nums[mid] == target {
            return mid as i32;
        }
        if nums[low] <= nums[mid] {
            if nums[low] <= target && target < nums[mid] {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        } else {
            if nums[mid] < target && target <= nums[high] {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
    }
    -1

}


pub fn duplicate_search(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    if n == 0 {
        return -1;
    }
    let mut low = 0;
    let mut high = n - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        if nums[mid] == target {
            return mid as i32;
        }
        if nums[low] == nums[mid] {
            low += 1;
        } else if nums[low] < nums[mid] {
            if nums[low] <= target && target < nums[mid] {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        } else {
            if nums[mid] < target && target <= nums[high] {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
    }
    -1
}

pub fn find_min(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 0 {
        return -1;
    }
    let mut low = 0;
    let mut high = n - 1;

    while low < high {
        let mid = low + (high - low) / 2;
        if nums[mid] > nums[high] {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    nums[low]
}

fn main() {
    let nums = vec![4,5,6,7,0,1,2];
    let target = 0;
    let index = unique_search(nums, target);
    println!("Target unique index: {}", index);

    let nums = vec![3,1,2,3,3,3,3];
    let target = 1;
    let index = duplicate_search(nums, target);
    println!("Target duplicate index: {}", index);

    let nums = vec![3,4,5,1,2];
    let min_value = find_min(nums);
    println!("Minimum value in rotated sorted array: {}", min_value);
}
