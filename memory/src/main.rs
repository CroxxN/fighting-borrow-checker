fn main() {
    let number = &42 as *const i32 as *mut i32;
    let deferenced = unsafe { *number + 2 };
    println!("{deferenced} also {}", unsafe { *number });
}
