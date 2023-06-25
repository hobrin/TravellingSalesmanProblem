use rand::prelude::*;
use itertools::Itertools;
use dialoguer::Confirm;

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
    println!("Average point: ({}, {})", avg_x, avg_y);
    
    let mut vec: Vec<usize> = (0..cities.len()).collect();
    vec.sort_by_key(|&idx| (((cities[idx].y-avg_y).atan2(cities[idx].x-avg_x)+100.0)*100.0) as usize);

    // TODO: sort the points based on their angle to the average point.
    vec
}
fn do_random(cities: &Vec<Point>) -> Vec<usize> {
    (0..cities.len()).collect()
}
fn do_closest_neighbour(cities: &Vec<Point>) -> Vec<usize> {
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
    let mut vec: Vec<usize> = vec![0, 1, 2]; // Pick 3 random points to start with.
    for _ in 0..(cities.len()-3) {
        let mut closest_interpoint = 0;
        let mut closest_dis = f64::INFINITY;
        let mut closest_city_idx = 0;

        for i in 0..vec.len() {
            let first = vec[if i>0 {i-1} else {vec.len()-1}];
            let second = vec[i];
            let avg_point = cities[first].average(&cities[second]);
            let close_point_idx = find_closest_point(&cities, &avg_point, &vec).unwrap();
            let close_dis = avg_point.distance(&cities[close_point_idx]);
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

struct AlgorithmStats {
    tot_dis: f64,
    tot_perfect: i64,
    tot_samples: i64,
}
impl Default for AlgorithmStats {
    fn default() -> Self {
        AlgorithmStats {
            tot_dis: 0.0,
            tot_perfect: 0,
            tot_samples: 0,
        }
    }
}

fn main() {
    println!("Welcome to the TSP solver!");
    let mut rng = rand::thread_rng();
    let mut stats: Vec<AlgorithmStats> = Vec::new();
    for i in 0..5 {
        stats.push(AlgorithmStats::default());
    }
    
    for _ in 0..100 {
        let mut cities: Vec<Point> = Vec::new();
        let n_cities: i32 = 9;
        for _ in 0..n_cities {
            let city: Point = Point {x: rng.gen_range(-10..10) as f64, y: rng.gen_range(-10..10) as f64};
            cities.push(city);
        }
        let answer = dis_in_path(&do_brute_force(&cities), &cities);

        stats[0].tot_dis += answer;
        stats[0].tot_perfect += 1;
        stats[0].tot_samples += 1;

        for (i, best) in vec![do_radial_algorithm(&cities), do_closest_neighbour(&cities), do_random(&cities), do_triangle_algorithm(&cities)].iter().enumerate() {
            let dis = dis_in_path(&best, &cities);

            println!("{}: distance is: {}", i, dis);
            stats[i+1].tot_dis += dis;
            stats[i+1].tot_perfect += (dis == answer) as i64;
            stats[i+1].tot_samples+=1;
            //for idx in best {
            //    let point = &cities[*idx];
            //    println!("({}, {})", point.x, point.y);
            //}
        }
    }
    for stat in &stats {
        println!(
            "tot_dis: {}, tot_perfect: {}, tot_samples: {}",
            stat.tot_dis, stat.tot_perfect, stat.tot_samples
        );
    }
}
