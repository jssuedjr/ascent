use ascent::ascent;

ascent! {

    relation edge(i32, i32);
    relation reachable(i32, i32);

    reachable(x, y) <-- edge(x, y);
    reachable(x, z) <-- edge(x, y), reachable(y, z);
}

fn main() {
    let mut prog = AscentProgram::default();

    prog.edge= vec![(1, 2),(2, 3),(3, 4)];

    prog.run();

    println!("Is 1 reachable from 4? {}", prog.reachable.contains(&(1, 4)));
    println!("Is 4 reachable from 1? {}", prog.reachable.contains(&(4, 1)));
}

