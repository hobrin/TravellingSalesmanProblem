use rand::prelude::*;
use itertools::Itertools;
use dialoguer::Confirm;
use std::time::{Instant, Duration};

struct Point {
    x: f64,
    y: f64,
}
fn factorial(n: usize) -> usize {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}
impl Point { 
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        ((dx*dx + dy*dy) as f64).sqrt()
    }
    fn average(&self, other: &Point) -> Point {
        Point {x: (self.x + other.x) / 2.0, y: (self.y + other.y) / 2.0}
    }
}
fn dis_in_path(vec: &Vec<usize>, cities: &Vec<Point>) -> f64 {
    let mut tot_dis: f64 = cities[vec[0 as usize] as usize].distance(&cities[vec[vec.len()-1] as usize]);
    for i in 0..(vec.len()-1) {
        tot_dis += cities[vec[i] as usize].distance(&cities[vec[i+1] as usize]);
    }
    tot_dis
}
fn find_closest_point(points: &Vec<Point>, target: &Point, exclude: &Vec<usize>) -> Option<usize> {
    let mut min_index = None;
    let mut min_dis = f64::INFINITY;

    for (index, point) in points.iter().enumerate() {
        let dis = point.distance(target);
        if dis < min_dis && !exclude.contains(&index) {
            min_dis = dis;
            min_index = Some(index);
        }
    }
    min_index
}

fn do_brute_force(cities: &Vec<Point>) -> Vec<usize> {
    let n_cities: usize = cities.len() as usize;
    let tot_ops = (n_cities-1)*(n_cities-2)/2*factorial(n_cities-3);
    //let confirmed = Confirm::new()
    //    .with_prompt(format!("This takes {} operations, are you sure?", tot_ops))
    //    .interact()
    //    .unwrap();
    //if !confirmed {
    //    println!("Alright, stopping!");
    //}

    let mut best: Vec<usize> = Vec::new();
    let mut best_dis: f64 = f64::MAX;
    for start in 1..n_cities {
        for end in (start+1)..n_cities {
            for permutation in (0..(n_cities-3)).collect::<Vec<usize>>().iter().permutations((n_cities-3) as usize) {
                let vec: Vec<&usize> = permutation.to_vec();
                let mut vec: Vec<usize> = vec.iter().map(|&x| *x + 1 + (*x + 1 >= start) as usize + (*x + 2 >= end) as usize).collect();
                vec.insert(0, start);
                vec.insert(0, 0);
                vec.push(end);
                
                let tot_dis: f64 = dis_in_path(&vec, &cities);

                if tot_dis < best_dis {
                    best_dis = tot_dis;
                    best = vec;
                }
            }
        }
    }
    best
}
fn do_radial_algorithm(cities: &Vec<Point>) -> Vec<usize> {
    let mut avg_x: f64 = cities.iter().map(|p| p.x).collect::<Vec<f64>>().iter().sum();
    avg_x = avg_x as f64 / cities.len() as f64;
    let mut avg_y: f64 = cities.iter().map(|p| p.y).collect::<Vec<f64>>().iter().sum();
    avg_y = avg_y as f64 / cities.len() as f64;
    //println!("Average point: ({}, {})", avg_x, avg_y);
    
    let mut vec: Vec<usize> = (0..cities.len()).collect();
    vec.sort_by_key(|&idx| (((cities[idx].y-avg_y).atan2(cities[idx].x-avg_x)+100.0)*100.0) as usize);

    // TODO: sort the points based on their angle to the average point.
    vec
}
fn do_random(cities: &Vec<Point>) -> Vec<usize> {
    (0..cities.len()).collect()
}
fn do_closest_neighbour(cities: &Vec<Point>) -> Vec<usize> { //Greedy algorithm.
    let mut vec: Vec<usize> = vec![0];
    for _ in 0..cities.len()-1 {
        vec.push(find_closest_point(&cities, &cities[vec[vec.len()-1]], &vec).unwrap());
    }
    vec
}
fn do_triangle_algorithm(cities: &Vec<Point>) -> Vec<usize> {
    if cities.len() <= 3 { // The algorithm doesn't work for <4 cities.
        return (0..cities.len()).collect(); //order doesn't matter.
    }
    let mut vec: Vec<usize> = Vec::new();
    let mut avg_x: f64 = cities.iter().map(|p| p.x).collect::<Vec<f64>>().iter().sum();
    avg_x = avg_x as f64 / cities.len() as f64;
    let mut avg_y: f64 = cities.iter().map(|p| p.y).collect::<Vec<f64>>().iter().sum();
    avg_y = avg_y as f64 / cities.len() as f64;
    let middle_avg: Point = Point {x: avg_x, y: avg_y};
    for i in 0..3 {
        vec.push(find_closest_point(&cities, &middle_avg, &vec).unwrap());
    }

    for _ in 0..(cities.len()-3) {
        let mut closest_interpoint = 0;
        let mut closest_dis = f64::INFINITY;
        let mut closest_city_idx = 0;

        for i in 0..vec.len() {
            let first = vec[if i>0 {i-1} else {vec.len()-1}];
            let second = vec[i];
            let avg_point = cities[first].average(&cities[second]);
            let close_point_idx = find_closest_point(&cities, &avg_point, &vec).unwrap();
            let close_dis = avg_point.distance(&cities[close_point_idx]) - avg_point.distance(&cities[first]);
            if close_dis < closest_dis {
                closest_dis = close_dis;
                closest_interpoint = i;
                closest_city_idx = close_point_idx;
            }
        }
        vec.insert(closest_interpoint, closest_city_idx);
        //println!("{:?}", vec);
    }
    //for point in cities {
    //    println!("({}, {})", point.x, point.y);
    //}
    vec
}
fn do_branch_and_bound(cities: &Vec<Point>) -> Vec<usize> {
    let n_cities = cities.len();
    let mut best_path = Vec::new();
    let mut best_distance = f64::MAX;
    let mut path = vec![0];

    let mut visited = vec![false; n_cities];
    visited[0] = true;

    branch_and_bound(
        cities,
        &mut path,
        &mut visited,
        0 as usize,
        0.0,
        &mut best_path,
        &mut best_distance,
    );

    best_path
}

fn branch_and_bound(
    cities: &Vec<Point>,
    path: &mut Vec<usize>,
    visited: &mut Vec<bool>,
    current_city: usize,
    current_distance: f64,
    best_path: &mut Vec<usize>,
    best_distance: &mut f64,
) {
    let n_cities = cities.len();

    if path.len() == n_cities {
        let distance_to_start = cities[current_city].distance(&cities[0]);
        let total_distance = current_distance + distance_to_start;

        if total_distance < *best_distance {
            *best_distance = total_distance;
            *best_path = path.clone();
        }
        return;
    }

    let mut candidates = Vec::new();

    for next_city in 0..n_cities {
        if !visited[next_city] {
            candidates.push(next_city);
        }
    }

    candidates.sort_by(|a, b| {
        let a_distance = cities[current_city].distance(&cities[*a]);
        let b_distance = cities[current_city].distance(&cities[*b]);
        a_distance.partial_cmp(&b_distance).unwrap()
    });

    for &next_city in &candidates {
        let distance = cities[current_city].distance(&cities[next_city]);

        if current_distance + distance >= *best_distance {
            break;
        }

        visited[next_city] = true;
        path.push(next_city);

        branch_and_bound(
            cities,
            path,
            visited,
            next_city,
            current_distance + distance,
            best_path,
            best_distance,
        ); path.pop();
        visited[next_city] = false;
    }
}

struct AlgorithmStats {
    tot_dis: f64,
    tot_perfect: i64,
    tot_samples: i64,
    tot_time: u128,
}
impl Default for AlgorithmStats {
    fn default() -> Self {
        AlgorithmStats {
            tot_dis: 0.0,
            tot_perfect: 0,
            tot_samples: 0,
            tot_time: 0,
        }
    }
}

fn main() {
    println!("Welcome to the TSP solver!");
    let mut rng = rand::thread_rng();
    let mut stats: Vec<AlgorithmStats> = Vec::new();
    for i in 0..6 {
        stats.push(AlgorithmStats::default());
    }
    
    for _ in 0..100 {
        let mut cities: Vec<Point> = Vec::new();
        let n_cities: i32 = 50;
        println!("a");
        for _ in 0..n_cities {
            let city: Point = Point {x: rng.gen_range(-10..10) as f64, y: rng.gen_range(-10..10) as f64};
            println!("({}, {})", city.x, city.y);
            cities.push(city);
        }
        let distance_matrix: Vec<Vec<Point>> = 

        let answer = 0.0;//dis_in_path(&do_brute_force(&cities), &cities);
        stats[0].tot_dis += answer;
        stats[0].tot_perfect += 1;
        stats[0].tot_samples += 1;
        

        let mut paths: Vec<Vec<usize>> = Vec::new();
        let time_radial = Instant::now();
        paths.push(do_radial_algorithm(&cities));
        stats[1].tot_time += time_radial.elapsed().as_nanos();
        let time_closest_neighbour = Instant::now();
        paths.push(do_closest_neighbour(&cities));
        stats[2].tot_time += time_closest_neighbour.elapsed().as_nanos();
        let time_random = Instant::now();
        paths.push(do_random(&cities));
        stats[3].tot_time += time_random.elapsed().as_nanos();
        let time_triangle_algorithm = Instant::now();
        paths.push(do_triangle_algorithm(&cities));
        stats[4].tot_time += time_triangle_algorithm.elapsed().as_nanos();

        for (i, best) in paths.iter().enumerate() {
            let dis = dis_in_path(&best, &cities);

            //println!("{}: distance is: {}", i, dis);
            stats[i+1].tot_dis += dis;
            stats[i+1].tot_perfect += (dis <= answer+0.01) as i64;
            stats[i+1].tot_samples+=1;
            //for idx in best {
            //    let point = &cities[*idx];
            //    println!("({}, {})", point.x, point.y);
            //}
        }
    }
    for stat in &stats {
        println!(
            "tot_dis: {}, tot_perfect: {}, tot_samples: {}, tot_time: {}",
            stat.tot_dis, stat.tot_perfect, stat.tot_samples, stat.tot_time/1000000u128
        );
    }
}

//TODO: Pre-compute the distance matrix.
