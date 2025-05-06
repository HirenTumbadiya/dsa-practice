pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let n1 = nums1.len();
    let n2 = nums2.len();

    if n1 > n2 {
        return find_median_sorted_arrays(nums2, nums1);
    }

    let total = n1 + n2;
    let half = (total + 1) / 2;
    let (mut low, mut high) = (0, n1);

    while low <= high {
        let mid1 = (low + high) / 2;
        let mid2 = half - mid1;

        let l1 = if mid1 == 0 { i32::MIN } else { nums1[mid1 - 1] };
        let l2 = if mid2 == 0 { i32::MIN } else { nums2[mid2 - 1] };
        let r1 = if mid1 == n1 { i32::MAX } else { nums1[mid1] };
        let r2 = if mid2 == n2 { i32::MAX } else { nums2[mid2] };

        if l1 <= r2 && l2 <= r1 {
            if total % 2 == 1 {
                return f64::from(l1.max(l2));
            } else {
                return (l1.max(l2) + r1.min(r2)) as f64 / 2.0;
            }
        } else if l1 > r2 {
            high = mid1 - 1;
        } else {
            low = mid1 + 1;
        }
    }

    0.0
        
}

fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    println!("Median: {}", find_median_sorted_arrays(nums1, nums2));
}
