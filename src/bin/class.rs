use ascent::ascent;

ascent! {

    relation prerequisite(&'static str, &'static str);
    relation must_take_before(&'static str, &'static str);

    must_take_before(c1, c2) <-- prerequisite(c1, c2);
    must_take_before(c1, c3) <-- prerequisite(c1, c2), must_take_before(c2, c3);
}

fn main() {
    let mut prog = AscentProgram::default();

    prog.prerequisite= vec![("Math", "Physics"),
    ("Physics", "Engineering")];

    prog.run();

    for (c1, c2) in prog.must_take_before.iter() {
        println!("You must take {} before {}", c1, c2);
    }
}

