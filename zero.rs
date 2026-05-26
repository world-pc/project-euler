//solution to problem zero

fn main() {
    let mut sum: i64 = 0;
    for i in 0..397000 {
        if i%2 != 0 {
            sum += (i as i64).pow(2);
        }
    }
    println!("{sum}");
}
