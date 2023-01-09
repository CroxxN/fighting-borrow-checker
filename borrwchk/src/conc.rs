fn main() {
    let mut local = 32;
    std::thread::scope(|s| {
        s.spawn(|| {
            local += 1;
        });
    });
    local += 1;
}
