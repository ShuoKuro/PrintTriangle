fn main() {
    let n: i32 = 5;
    for n in 0..n {
        for n in 0..n + 1 {
            print!("&");
        }
        print!("\n");
    }
}
