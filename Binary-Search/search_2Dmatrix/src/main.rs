pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut low = 0;
    let mut high = rows * cols - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let mid_value = matrix[mid / cols][mid % cols];

        if mid_value == target {
            return true;
        } else if mid_value < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    false
}

fn main() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let target = 3;
    println!("Found target? {}", search_matrix(matrix, target));
}
