use std::str::FromStr;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::time::{Instant};

#[derive(Debug)]
pub struct Cave {
    risk_levels: Vec<Vec<u8>>,
    dims: (usize, usize),
}

pub fn parse_cave(array: &str) -> Cave {
    let mut risk_levels = vec![];
    for line in array.split("\n") {
        if line.is_empty() {
            continue;
        }
        let mut row = vec![];
        for digit in line.split("") {
            if !digit.is_empty() {
                row.push(u8::from_str(digit).unwrap());
            }
        }
        risk_levels.push(row);
    }
    let n = risk_levels.len();
    let m = risk_levels[1].len();
    Cave { risk_levels, dims: (n, m) }
}

type PrevMap = Vec<Vec<Option<(usize, usize)>>>;

#[derive(PartialEq, Eq)]
struct PrioritizedNode {
    i: usize,
    j: usize,
    distance: usize,
}

impl Ord for PrioritizedNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
                      .then_with(|| self.i.cmp(&other.i))
                      .then_with(|| self.j.cmp(&other.j))
    }
}

impl PartialOrd for PrioritizedNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn best_path_through(cave: &Cave) -> usize {
    let (n, m) = cave.dims;
    let mut visited = vec![vec![false; m]; n];
    let mut heap = BinaryHeap::new();
    let mut distances = vec![vec![usize::MAX; m]; n];
    distances[0][0] = 0;

    for i in 0..n {
        for j in 0..m {
            if (i, j) == (0, 0) {
                heap.push(PrioritizedNode { i, j, distance: 0 });
            } else {
                heap.push(PrioritizedNode { i, j, distance: usize::MAX });
            }
        }
    }

    let mut predecessors: PrevMap = vec![vec![None; m]; n];

    while let Some(pn @ PrioritizedNode { i, j, distance: current_dist }) = heap.pop() {
        if (i, j) == (n-1, m-1) {
            break;
        }

        for (d1, d2) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let k = i as i64 + d1;
            let l = j as i64 + d2;
            if k < 0 || k >= n as i64 {
                continue;
            }
            if l < 0 || l >= m as i64 {
                continue;
            }
            let k = k as usize;
            let l = l as usize;
            if visited[k][l] {
                continue;
            }

            let best_distance_so_far = distances[k][l];
            let distance_through_current = current_dist + cave.risk_levels[k][l] as usize;
            if distance_through_current < best_distance_so_far {
                predecessors[k][l] = Some((i, j));
                let updated_neighbor = PrioritizedNode {
                    i: k, j: l,
                    distance: distance_through_current
                };
                distances[k][l] = distance_through_current;
                heap.push(updated_neighbor);
            }
        }
        visited[i][j] = true;
    }

    let mut last = (n-1, m-1);
    let mut total_risk = 0;
    while last != (0, 0) {
        let (i, j) = last;
        let risk = cave.risk_levels[i][j];
        total_risk += risk as usize;
        last = predecessors[i][j].unwrap();
    }
    total_risk
}

fn clamp(r: u8) -> u8 {
    ((r-1) % 9) + 1
}

pub fn part2(cave: &Cave) -> Cave {
    let array = &cave.risk_levels;
    let (n, m) = cave.dims;
    let mut new_array = vec![vec![0; 5*m]; 5*n];
    for k in 0..5 {
        for l in 0..5 {
            for i in 0..n {
                for j in 0..m {
                    let s = k*n + i;
                    let t = l*m + j;
                    new_array[s][t] = clamp((array[i][j] as usize + k + l) as u8);
                }
            }
        }
    }
    Cave {
        dims: (5*n, 5*m),
        risk_levels: new_array,
    }
}

pub fn run() {
    let test = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    let cave = parse_cave(test);
    best_path_through(&cave);

    let part2_cave = part2(&cave);
    best_path_through(&part2_cave);

    let input = include_str!("../resources/day15_input.txt");
    let cave = parse_cave(input);
    dbg!(&cave.dims);
    best_path_through(&cave);

    let part2_cave = part2(&cave);

    let start = Instant::now();
    let answer = best_path_through(&part2_cave);
    let duration = start.elapsed();
    dbg!(answer);
    println!("Time elapsed in part2 is: {:?}", duration);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_clamp() {
        assert!(clamp(9+1) == 1);
        assert!(clamp(8+4) == 3);
        assert!(clamp(8+8) == 7);
    }
}
