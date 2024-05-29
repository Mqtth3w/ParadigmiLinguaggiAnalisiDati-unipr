use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::time::Instant;
use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Edge {
    node: usize,
    cost: u32,
}

#[derive(Debug, Clone)]
struct Node {
    edges: Vec<Edge>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse order for min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra_seq(graph: &Vec<Node>, start: usize) -> Vec<u32> {
    let mut dist: Vec<u32> = (0..graph.len()).map(|_| u32::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist[position] {
            continue;
        }

        for edge in &graph[position].edges {
            let next = State { cost: cost + edge.cost, position: edge.node };

            if next.cost < dist[edge.node] {
                dist[edge.node] = next.cost;
                heap.push(next);
            }
        }
    }

    dist
}

fn dijkstra_par(graph: &Vec<Node>, start: usize) -> Vec<u32> {
    let mut dist: Vec<u32> = (0..graph.len()).map(|_| u32::MAX).collect();
    let mut heap = BinaryHeap::new();
    let mut visited: Vec<bool> = vec![false; graph.len()];

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if visited[position] {
            continue;
        }
        visited[position] = true;

        let neighbors: Vec<_> = graph[position].edges.par_iter()
            .map(|edge| State { cost: cost + edge.cost, position: edge.node })
            .collect();

        for next in neighbors {
            if next.cost < dist[next.position] {
                dist[next.position] = next.cost;
                heap.push(next);
            }
        }
    }

    dist
}

fn main() {
    // Example graph: 0 --1--> 1 --2--> 2 --1--> 3
    let graph = vec![
        Node { edges: vec![Edge { node: 1, cost: 1 }] },
        Node { edges: vec![Edge { node: 2, cost: 2 }] },
        Node { edges: vec![Edge { node: 3, cost: 6 }] },
        Node { edges: vec![Edge { node: 4, cost: 6 }] },
        Node { edges: vec![Edge { node: 5, cost: 4 }] },
        Node { edges: vec![Edge { node: 6, cost: 3 }] },
        Node { edges: vec![Edge { node: 7, cost: 8 }] },
        Node { edges: vec![Edge { node: 8, cost: 7 }] },
        Node { edges: vec![Edge { node: 9, cost: 2 }] },
        Node { edges: vec![Edge { node: 10, cost: 11 }] },
        Node { edges: vec![] },
    ];

    let start_node = 0;

    // Sequential execution
    let start = Instant::now();
    let distances_seq = dijkstra_seq(&graph, start_node);
    let duration = start.elapsed();
    println!("Sequential execution time: {:?}", duration);
    println!("Distances (sequential): {:?}", distances_seq);

    // Parallel execution
    let start = Instant::now();
    let distances_par = dijkstra_par(&graph, start_node);
    let duration = start.elapsed();
    println!("Parallel execution time: {:?}", duration);
    println!("Distances (parallel): {:?}", distances_par);

    // Optional: Compare results
    for (i, (d_seq, d_par)) in distances_seq.iter().zip(distances_par.iter()).enumerate() {
        assert_eq!(d_seq, d_par, "Mismatch at node {}: seq ({:?}), par ({:?})", i, d_seq, d_par);
    }

    println!("The sequential and parallel results match.");
}
