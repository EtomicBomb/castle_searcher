use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::hash::Hash;
use std::fmt::Debug;

const N_TRAINING_SAMPLES: usize = 100_000;

fn main() {
    let mut searcher = CastleSearcher::new();
    println!("{:?}", searcher.pathfind());
}


impl Searchable for CastleSearcher {
    type Solution = Castle;

    fn output(&mut self, solution: &Self::Solution, fitness: f64) {
        if let Some(boat) = self.best_of_all_time.peek() {
            println!("{}:  {:?} - best: {}: {:?}", fitness, solution.troops(), boat.value, boat.item.troops());

        }
    }

    fn neighbors(&self, solution: &Self::Solution) -> Vec<Self::Solution> {
        solution.neighbors()
    }

    fn fitness_estimate(&mut self, solution: &Self::Solution) -> f64 {
        let fitness = self.test_on_data(solution) as f64;

        if let Some(boat) = self.best_of_all_time.peek() {
            if fitness > boat.value {
                self.best_of_all_time.push(HeapEntry { value: fitness, item: solution.clone() });
            }
        } else {
            self.best_of_all_time.push(HeapEntry { value: fitness, item: solution.clone() });
        }


        fitness
    }

    fn is_goal(&self, solution: &Self::Solution, fitness: f64) -> bool {
        false
    }

    fn start(&self) -> Self::Solution {
        Castle::from_random()
        //Castle::default()
    }
}



#[derive(Clone)]
struct CastleSearcher {
    best_of_all_time: BinaryHeap<HeapEntry<Castle>>,
    training_data: Vec<Castle>,
}

impl CastleSearcher {
    fn new() -> CastleSearcher {
        let mut training_data = Vec::new();

        for _ in 0..N_TRAINING_SAMPLES {
            training_data.push(Castle::from_random());
        }

        CastleSearcher {
            best_of_all_time: BinaryHeap::new(),
            training_data
        }
    }

    fn test_on_data(&self, solution: &Castle) -> usize {
        let mut wins = 0;

        for other in self.training_data.iter() {
            if solution.does_win(other) { wins += 1 }
        }

        wins
    }
}



#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
struct Castle {
    inner: [u8; 9],
}

impl Castle {

    fn does_win(&self, other: &Self) -> bool {
        let mut self_points = 0;
        let mut other_points = 0;

        for (i, (s, o)) in self.troops().iter().zip(other.troops().iter()).enumerate() {
            let i = i+1;

            match s.cmp(o) {
                Ordering::Less => other_points += 2*i,
                Ordering::Equal => {
                    self_points += i;
                    other_points += i;
                },
                Ordering::Greater => self_points += 2*i,
            }
        }

        self_points > other_points
    }

    fn troops(&self) -> Vec<u8> {
        let mut troops = Vec::new();
        troops.push(self.inner[0]);


        for i in 0..8 {
            troops.push(self.inner[i+1]-self.inner[i]);
        }

        troops.push(100-self.inner.last().unwrap());

        troops
    }

    fn from_random() -> Castle {
        let mut inner = [0; 9];

        for i in 0..9 {
            inner[i] = thread_rng().gen_range(0, 100);
        }

        inner.sort();

        Castle { inner }
    }

    fn neighbors(&self) -> Vec<Castle> {
        let mut neighbors = Vec::new();

        for update_index in 0..9 {
            for &d in [-1, 1].iter() {
                let mut current_value = self.inner[update_index] as i32 + d;

                if current_value < 0 || current_value > 100 {
                    continue;
                }

                let mut new = self.clone();
                new.inner[update_index] = current_value as u8;

                new.inner.sort();

                neighbors.push(new);
            }
        }

        neighbors
    }
}


trait Searchable: Clone {
    type Solution: Clone+Eq+Hash+Debug;

    fn neighbors(&self, solution: &Self::Solution) -> Vec<Self::Solution>;

    fn fitness_estimate(&mut self, solution: &Self::Solution) -> f64;

    fn is_goal(&self, solution: &Self::Solution, fitness: f64) -> bool;

    fn start(&self) -> Self::Solution;

    fn output(&mut self, solution: &Self::Solution, fitness: f64);


    fn heap_entry(&mut self, solution: Self::Solution) -> HeapEntry<Self::Solution> {
        HeapEntry {
            value: self.fitness_estimate(&solution),
            item: solution,
        }
    }

    fn pathfind(&mut self) -> Self::Solution {
        let mut open = BinaryHeap::new();
        let mut seen = HashSet::new();

        let start = self.start();
        seen.insert(start.clone());
        open.push(self.heap_entry(start.clone()));


        while let Some(current_heap_entry) = open.pop() {
            let HeapEntry { item: current, value: current_fitness } = current_heap_entry;

            self.output(&current, current_fitness);

            if self.is_goal(&current, current_fitness) {
                return current.clone();
            }

            for neighbor in self.neighbors(&current) {
                if seen.contains(&neighbor) { continue }

                seen.insert(neighbor.clone());
                open.push(self.heap_entry(neighbor.clone()));
            }
        }

        unreachable!()
    }
}

struct HeapEntry<T> {
    value: f64,
    item: T,
}

impl<T: Clone> Clone for HeapEntry<T> {
    fn clone(&self) -> HeapEntry<T> {
        HeapEntry {
            value: self.value,
            item: self.item.clone(),
        }
    }
}

impl<T> PartialOrd for HeapEntry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T> Ord for HeapEntry<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T> PartialEq for HeapEntry<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for HeapEntry<T> { }
