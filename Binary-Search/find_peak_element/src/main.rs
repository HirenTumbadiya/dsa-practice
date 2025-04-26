pub fn single_peak_element(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
        return nums[0];
    }

    if nums[0] > nums[1] {
        return nums[0];
    }
    if nums[n - 1] > nums[n - 2] {
        return nums[n - 1];
    }
    let mut low = 1;
    let mut high = n - 2;

    while low <= high {
        let mid = low + (high - low) / 2;
        if nums[mid] > nums[mid - 1] && nums[mid] > nums[mid + 1] {
            return nums[mid];
        } else if nums[mid] < nums[mid - 1] {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    -1
}

pub fn multiple_peak_elements(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut peaks = Vec::new();

    if n == 0 {
        return peaks;
    }

    if n == 1 {
        peaks.push(nums[0]);
        return peaks;
    }

    if nums[0] > nums[1] {
        peaks.push(nums[0]);
    }
    if nums[n - 1] > nums[n - 2] {
        peaks.push(nums[n - 1]);
    }

    for i in 1..(n - 1) {
        if nums[i] > nums[i - 1] && nums[i] > nums[i + 1] {
            peaks.push(nums[i]);
        }
    }

    peaks
}


fn main() {
    let nums = vec![1, 2, 3, 1];
    let peak = single_peak_element(nums);
    println!("The peak element is: {}", peak);

    let nums = vec![1, 2, 3, 1, 2, 3, 4, 5, 4, 5, 6, 5];
    let peaks = multiple_peak_elements(nums);
    println!("The peak elements are: {:?}", peaks);
}
