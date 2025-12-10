fn main() {
    let mut x = "XYZ";
    println!("x is {}", x);

    let x = 45; // It shadows previous x (i.e. x="XYZ" )
    println!("x is {}", x);

    let x = true; // It shadows previous x (i.e. x=45 )
    println!("x is {}", x);

    // -------------------------
    // It works like this:
    // -------------------------
    let mut x = "XYZ";
    {
        let x = 45;
        {
            let x = true;
        }
    }
}
