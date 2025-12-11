// ============================================================
//   RUST BORROWING RULES
//      1. Every value has varible which is its owner
//      2. There can only be one owner at a time
//      3. When owner goes out of scope, the value will be dropped
// ============================================================

fn main() {
    // ============================================================
    //      CASE 1: MULTIPLE IMMUTABLE BORROWS - ALLOWED
    // ============================================================
    {
        let s = String::from("hello");

        let r1 = &s;
        let r2 = &s;

        // Both are allowed because they are READ-ONLY borrows.
        println!("r1 = {}, r2 = {}", r1, r2);
    }

    // ============================================================
    //      CASE 2: TWO MUTABLE BORROWS - NOT ALLOWED
    //      -> AT A TIME, ONLY ONE MUTABLE BORROW CAN EXIST
    // ============================================================
    {
        let mut s = String::from("hello");

        let _r1 = &mut s;
        // let _r2 = &mut s; // ERROR
        // println!("{} - {}", _r1, _r2);

        // ❌ ERROR:
        // cannot borrow `s` as mutable more than once at a time
        println!("Only one mutable borrow allowed at a time.");
    }

    // =======================================================================
    //      CASE 3: IMMUTABLE + MULTIPLE BORROWS AT SAME TIME - NOT ALLOWED
    //      -> READER CAN READ PARTIAL/HALF MODIFIED DATA
    // =======================================================================
    {
        let mut s = String::from("hello");

        let r1 = &s;
        // let r2 = &mut s;
        // ❌ ERROR:
        // cannot borrow `s` as mutable because it is also borrowed as immutable

        println!("r1 = {}", r1);
    }

    // =============================================================================================
    //      CASE 4: IMMUTABLE ENDS BEFORE MULTIPLE BEGINS - ALLOWED by NLL(NON-LEXICAL LIFETIMES)
    //      -> REFRENCES ENDS WHEN IT IS NOT IN USED, NOT WHEN IT GOES OUT OF THE SCOPE
    // =============================================================================================
    {
        let mut s = String::from("hello");

        let r1 = &s;
        println!("Last use of r1: {}", r1);
        // r1’s lifetime ends RIGHT HERE due to NLL

        let r2 = &mut s; // ✔ Allowed because r1 is no longer used
        r2.push_str(" world");

        println!("After mutation: {}", s);
    }

    // =============================================================================================
    //      CASE 5: TWO MULTIPLE BORROWS IN DIFFN SCOPES - ALLOWED
    // =============================================================================================
    {
        let mut s = String::from("hello");

        {
            let r1 = &mut s;
            r1.push_str("!");
            println!("After r1: {}", r1);
        } // r1 DROPPED here

        {
            let r2 = &mut s;
            r2.push_str("!!");
            println!("After r2: {}", r2);
        } // r2 DROPPED here

        println!("Final: {}", s);
    }
}
