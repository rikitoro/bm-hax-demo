use bm_hax_demo::majority_candidate;

fn main() {
    let votes = [2, 2, 1, 1, 1, 2, 2];
    let candidate = majority_candidate(&votes);
    println!("votes     = {:?}", votes);
    println!("candidate = {}", candidate);
}