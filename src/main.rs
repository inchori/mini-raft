use mini_raft::timer::{Timer, random_election_timeout};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // 타이머 테스트
    let timeout = random_election_timeout();
    println!("Election timeout: {:?}", timeout);
    
    let mut timer = Timer::new(Duration::from_millis(100));
    println!("Timer created, elapsed: {}", timer.is_elapsed());
    
    sleep(Duration::from_millis(150));
    println!("After 150ms, elapsed: {}", timer.is_elapsed());
    
    timer.reset();
    println!("After reset, elapsed: {}", timer.is_elapsed());
}
