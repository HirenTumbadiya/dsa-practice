fn minimise_max_distance(stations: Vec<f64>, k: i32) -> f64 {
    fn is_possible(stations: &Vec<f64>, k: i32, dist: f64) -> bool {
        let mut count = 0;
        for i in 1..stations.len() {
            let d = stations[i] - stations[i - 1];
            count += (d / dist).floor() as i32;
        }
        count <= k
    }

    let mut left = 0.0;
    let mut right = stations.windows(2).map(|w| w[1] - w[0]).fold(0.0, f64::max);
    let eps = 1e-6;

    while right - left > eps {
        let mid = (left + right) / 2.0;
        if is_possible(&stations, k, mid) {
            right = mid;
        } else {
            left = mid;
        }
    }

    left
}





// Brute force solution

fn is_possible_brute(stations: &Vec<f64>, k: i32, dist: f64) -> bool {
    let mut count = 0;
    for i in 1..stations.len() {
        let gap = stations[i] - stations[i - 1];
        count += (gap / dist).floor() as i32;
    }
    count <= k
}

fn brute_force_min_distance(stations: Vec<f64>, k: i32) -> f64 {
    let mut best = f64::MAX;

    let max_gap = stations.windows(2).map(|w| w[1] - w[0]).fold(0.0, f64::max);

    let mut d = 0.01;
    while d <= max_gap {
        if is_possible_brute(&stations, k, d) {
            best = d;
            break;
        }
        d += 0.01; // step size controls precision
    }

    best
}

fn main() {
    let stations = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let k = 6;
    let result = minimise_max_distance(stations, k);
    println!("Minimum possible maximum distance: {:.6}", result);

    let stations = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let k = 6;
    let result = brute_force_min_distance(stations, k);
    println!("Brute force minimum possible max distance: {:.2}", result);
}
