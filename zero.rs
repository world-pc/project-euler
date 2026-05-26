//solution to problem zero

fn main() {
    let mut sum: i64 = 0;
    for i in (1..397000).step_by(2) {
        sum += (i as i64).pow(2);
    }
    println!("{sum}");
}
