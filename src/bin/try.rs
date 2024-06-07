use ascent::ascent;
use ascent::Dual;

ascent! {

    relation edge(i32, i32, u32); // edge(from, to, weight)

    relation longest_path(i32, i32, Dual<u32>); // longest_path(from, to, distance)

    longest_path(x, y, Dual(*w)) <-- edge(x, y, w);

    longest_path(x, z, Dual(w + l)) <--
        edge(x, y, w),
        longest_path(y, z, ?Dual(l));
}

fn main() {
    let mut prog = AscentProgram::default();

    prog.edge= vec![(1, 2, 3), (1, 3, 1), (2, 3, 1), (3, 4, 2), (2, 4, 4),];

    prog.run();

    for (x, y, Dual(dist)) in &prog.longest_path {
        println!("Longest path from {} to {} is {}", x, y, dist);
    }
}

