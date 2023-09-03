struct Memory {
    storge: i32,
}

fn main() {
    const _THISER: Memory = Memory { storge: 32 };
    let number = &42 as *const i32 as *mut i32;
    let numberical = &34;
    let _num = numberical as *const i32 as *mut i32;
    let deferenced = unsafe { *number + 2 };
    println!("{deferenced} also {}", unsafe { *number });
}

fn _make() {
    let _this = &mut "cout";
}
