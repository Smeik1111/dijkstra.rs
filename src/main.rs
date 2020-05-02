mod graph;
extern crate rand;

fn main() {
    let sample = graph::sample();
    println!("{}", sample.json());
}
