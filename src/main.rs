use mini_raft::types::Term;

fn main() {
    let term = Term::new(1);
    println!("term: {:?}", term);
}
