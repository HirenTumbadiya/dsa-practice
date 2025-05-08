fn median(matrix: Vec<Vec<i32>>) -> i32 {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut min = matrix[0][0];
    let mut max = matrix[0][cols - 1];
    
    for i in 1..rows {
        min = min.min(matrix[i][0]);
        max = max.max(matrix[i][cols - 1]);
    }

    let desired = (rows * cols) / 2;

    while min < max {
        let mid = min + (max - min) / 2;
        let mut count = 0;

        for i in 0..rows {
            count += upper_bound(&matrix[i], mid);
        }

        if count <= desired {
            min = mid + 1;
        } else {
            max = mid;
        }
    }

    min
}

// Returns the count of numbers less than or equal to target using upper_bound logic
fn upper_bound(row: &Vec<i32>, target: i32) -> usize {
    let mut low = 0;
    let mut high = row.len();

    while low < high {
        let mid = (low + high) / 2;
        if row[mid] <= target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    low
}

fn main() {
    let matrix = vec![
        vec![1, 3, 5],
        vec![2, 6, 9],
        vec![3, 6, 9],
    ];

    let result = median(matrix);
    println!("Median is: {}", result);
}
