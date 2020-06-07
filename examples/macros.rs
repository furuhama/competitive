use competitive::{echo, max, min};

fn main() {
    let max = max!(1, 2, 3);
    let min = min!(1, 2, 3);

    echo!(max);
    echo!(min);
}
