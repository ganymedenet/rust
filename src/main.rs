use std::time::Instant;

fn main(){
    let now = Instant::now();

    let mut n:i32 = 1;
    let mut x:i32 = 0;

    while n < 99999999 {
        x = 999 * 999;
        n += 1;
    }

    println!("{}\n", now.elapsed().as_secs_f32());
    println!("{}", x);

}