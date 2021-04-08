use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct WithLife<'a> {
    s: &'a String,

}

#[derive(Debug)]
pub struct NoLife {
    s: Rc<RefCell<String>>,
}

fn main() -> Result<(), std::io::Error> {
    // let (l1, l2) = make_with_life("test_data/v3_data.txt")?;
    let (l1, l2) = make_no_life("test_data/v3_data.txt")?;

    let mut s = l1.s.borrow_mut();
    // already borrowed error
    // let s2 = l2.s.borrow();
    s.push_str("What if it was even bigger");

    // not has to give data
    println!("l1 = {:?}", l1);
    println!("l2 = {:?}", l2);

    println!("s == {}", s);
    drop(s);

    // can see value again with new line
    println!("l1 = {:?}", l1);
    println!("l2 = {:?}", l2);

    Ok(())
}

// fn make_with_life<'a>(fname: &str) -> Result<(WithLife<'a>, WithLife<'a>), std::io::Error> {
//     let s = std::fs::read_to_string(fname)?;
//     Ok((WithLife{s: &s}, WithLife{s: &s}))
// }

fn make_no_life(fname: &str) -> Result<(NoLife, NoLife), std::io::Error> {
    let s = std::fs::read_to_string(fname)?;
    let r = Rc::new(RefCell::new(s));
    Ok((NoLife{s: r.clone()}, NoLife{s: r}))
}