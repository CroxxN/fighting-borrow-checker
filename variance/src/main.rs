// Runs
fn maker(x: &mut &str, y: &'static str) {
    *x = y;
}

fn main() {
    let xtr = &mut "hey";
    static YTR: &'static str = "hello";
    maker(xtr, YTR);
}
