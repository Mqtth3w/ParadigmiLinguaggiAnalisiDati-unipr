use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

#[derive(Clone, Copy, Debug)]
struct Body {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    mass: f64,
}

impl Body {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Body {
            x: rng.gen_range(-1.0..1.0),
            y: rng.gen_range(-1.0..1.0),
            vx: 0.0,
            vy: 0.0,
            mass: rng.gen_range(0.1..10.0),
        }
    }
}

fn calculate_forces_seq(bodies: &mut [Body], dt: f64) {
    let g = 6.67430e-11;

    for i in 0..bodies.len() {
        let mut fx = 0.0;
        let mut fy = 0.0;

        for j in 0..bodies.len() {
            if i != j {
                let dx = bodies[j].x - bodies[i].x;
                let dy = bodies[j].y - bodies[i].y;
                let dist = (dx * dx + dy * dy).sqrt();
                let force = g * bodies[i].mass * bodies[j].mass / (dist * dist);
                fx += force * dx / dist;
                fy += force * dy / dist;
            }
        }

        bodies[i].vx += fx / bodies[i].mass * dt;
        bodies[i].vy += fy / bodies[i].mass * dt;
    }

    for body in bodies.iter_mut() {
        body.x += body.vx * dt;
        body.y += body.vy * dt;
    }
}

fn calculate_forces_par(bodies: &mut [Body], dt: f64) {
    let g = 6.67430e-11;

    let forces: Vec<(f64, f64)> = bodies
        .par_iter()
        .map(|body_i| {
            let mut fx = 0.0;
            let mut fy = 0.0;

            for body_j in bodies.iter() {
                if body_i as *const _ != body_j as *const _ {
                    let dx = body_j.x - body_i.x;
                    let dy = body_j.y - body_i.y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    let force = g * body_i.mass * body_j.mass / (dist * dist);
                    fx += force * dx / dist;
                    fy += force * dy / dist;
                }
            }

            (fx, fy)
        })
        .collect();

    bodies
        .par_iter_mut()
        .zip(forces)
        .for_each(|(body, (fx, fy))| {
            body.vx += fx / body.mass * dt;
            body.vy += fy / body.mass * dt;
        });

    bodies.par_iter_mut().for_each(|body| {
        body.x += body.vx * dt;
        body.y += body.vy * dt;
    });
}

fn main() {
    let n_bodies = 10000;
    let mut bodies_seq: Vec<Body> = (0..n_bodies).map(|_| Body::new()).collect();
    let mut bodies_par = bodies_seq.clone();
    let dt = 0.01;

    // Sequential execution
    let start = Instant::now();
    calculate_forces_seq(&mut bodies_seq, dt);
    let duration = start.elapsed();
    println!("Sequential execution time: {:?}", duration);

    // Parallel execution
    let start = Instant::now();
    calculate_forces_par(&mut bodies_par, dt);
    let duration = start.elapsed();
    println!("Parallel execution time: {:?}", duration);

    // Optional: Compare results
    for (i, (b_seq, b_par)) in bodies_seq.iter().zip(bodies_par.iter()).enumerate() {
        assert!(
            (b_seq.x - b_par.x).abs() < 1e-10 && (b_seq.y - b_par.y).abs() < 1e-10,
            "Mismatch at body {}: seq ({:.10}, {:.10}), par ({:.10}, {:.10})",
            i,
            b_seq.x,
            b_seq.y,
            b_par.x,
            b_par.y
        );
    }

    println!("The sequential and parallel results match.");
}
