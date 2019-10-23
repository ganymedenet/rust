use std::time::Instant;

fn main(){
    let now = Instant::now();

    let mut n = 1;
    let mut x:i128 = 5;

    while n < 9999999 {
        x = 999 * 999;
        n += 1;
    }

    println!("{}\n", now.elapsed().as_secs_f32());
    println!("{}", x);

}