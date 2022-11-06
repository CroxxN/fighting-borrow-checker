fn main() {
    let base = -50_i32;
    let positive_base = 100_i32;
    let number = !(base as u32) + 1;
    let postive = (!(positive_base as u32) + 1) as i32;
    println!("{number} positive conversion: {postive}");

}
