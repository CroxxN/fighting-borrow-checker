mod conc;

struct Arg {
    make: &Vec<i32>,
    // vi: Vec<i32>,
}

fn return_ref(ar: &Arg) -> &i32 {
    ar.make.get(1).unwrap().
}

fn main() {
    let mut this: Arg = Arg {
        make: &vec![1, 2, 3],
    };
    let miss = return_ref(&this);
    let shit = vec![3, 4, 5];
    this.make = &shit;
    println!("{miss:#?}");
    println!("{:#?}", this.make);
}
