use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::time::Instant;
use rand::Rng;
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
    let num_nodes = 1000;
    let mut rng = rand::thread_rng();

    // Generate a graph with #num_nodes
    let graph: Vec<Node> = (0..num_nodes).map(|_| {
        let num_edges = rng.gen_range(1..10); // Each node has between 1 and 10 edges
        let edges = (0..num_edges).map(|_| {
            Edge {
                node: rng.gen_range(0..num_nodes),
                cost: rng.gen_range(1..20), // Cost between 1 and 20
            }
        }).collect();
        Node { edges }
    }).collect();

    let start_node = 0;

    // Sequential execution
    let start = Instant::now();
    let distances_seq = dijkstra_seq(&graph, start_node);
    let duration = start.elapsed();
    println!("Sequential execution time: {:?}", duration);
    println!("Distances (sequential): {:?}", &distances_seq[..10]); // Print first 10 distances

    // Parallel execution
    let start = Instant::now();
    let distances_par = dijkstra_par(&graph, start_node);
    let duration = start.elapsed();
    println!("Parallel execution time: {:?}", duration);
    println!("Distances (parallel): {:?}", &distances_par[..10]); // Print first 10 distances

    // Optional: Compare results
    for (i, (d_seq, d_par)) in distances_seq.iter().zip(distances_par.iter()).enumerate() {
        assert_eq!(d_seq, d_par, "Mismatch at node {}: seq ({:?}), par ({:?})", i, d_seq, d_par);
    }

    println!("The sequential and parallel results match.");
}
