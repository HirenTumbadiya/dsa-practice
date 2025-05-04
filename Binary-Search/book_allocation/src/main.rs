fn book_allocation(books: &[i32], students: i32) -> i32 {
    let mut low = *books.iter().max().unwrap();
    let mut high = books.iter().sum();
    let mut result = high;

    while low <= high {
        let mid = (low + high) / 2;
        if is_possible(books, students, mid) {
            result = mid;
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    result
}
fn is_possible(books: &[i32], students: i32, max_pages: i32) -> bool {
    let mut student_count = 1;
    let mut pages_sum = 0;

    for &pages in books {
        if pages_sum + pages > max_pages {
            student_count += 1;
            pages_sum = pages;
            if student_count > students {
                return false;
            }
        } else {
            pages_sum += pages;
        }
    }
    true
}

fn main() {
    let books = [12, 34, 67, 90];
    let students = 2;
    let result = book_allocation(&books, students);
    println!("Minimum number of pages: {}", result);
}
