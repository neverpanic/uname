extern crate uname;

use uname::uname;

fn main() {
    let info = uname().unwrap();

    println!("{:?}", info)
}
