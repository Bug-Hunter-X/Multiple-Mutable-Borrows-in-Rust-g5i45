fn main() {
    let mut x = 5;
    { //Use a block to limit the scope of the mutable borrow
        let y = &mut x;
        *y += 1;
    }
    { //Use another block to limit the scope of the mutable borrow
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x);
}
