pub fn find_kth_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>, k: usize) -> f64 {
    let (a, b) = (nums1, nums2);
    let (n1, n2) = (a.len(), b.len());

    if n1 > n2 {
        return find_kth_sorted_arrays(b, a, k);
    }

    let mut low = 0;
    let mut high = n1.min(k);

    while low <= high {
        let cut1 = (low + high) / 2;
        let cut2 = k - cut1;

        let l1 = if cut1 == 0 { i32::MIN } else { a[cut1 - 1] };
        let l2 = if cut2 == 0 { i32::MIN } else { b[cut2 - 1] };
        let r1 = if cut1 == n1 { i32::MAX } else { a[cut1] };
        let r2 = if cut2 == n2 { i32::MAX } else { b[cut2] };

        if l1 <= r2 && l2 <= r1 {
            return l1.max(l2) as f64;
        } else if l1 > r2 {
            high = cut1 - 1;
        } else {
            low = cut1 + 1;
        }
    }

    return -1.0;
        
}

fn main() {
    let nums1 = vec![1, 3, 5];
    let nums2 = vec![2, 4, 6, 8];
    let k = 5;
    let kth = find_kth_sorted_arrays(nums1, nums2, k);
    println!("The {}-th smallest element is: {}", k, kth);
}
