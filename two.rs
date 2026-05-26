//solution to problem two

fn main() {
    let mut a = 1;
    let mut b = 1;
    let mut sum = 0;

    while b < 4000000 {
        if b % 2 == 0 {
            sum += b;
        }

        //gen the next fibonacci number
        let c = a;
        a = b;
        b += c;
    }
    
    println!("{sum}");
}
