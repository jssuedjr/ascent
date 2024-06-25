use ascent::ascent;
use ascent::Dual;

// based on a given sequence of instructions, determine if the sequence is valid

ascent! {
    // Define instruction relations
    relation mov(&'static str, &'static str); 
    relation je(i32, &'static str); // if result of prior arithmetic operation is equal to zero, jump to address
    relation add(&'static str, &'static str);
    relation sub(&'static str, &'static str);
    lattice valid_sequence(,,Dual<>);
}

fn main() {
    let mut prog: AscentProgram = AscentProgram::default();
    prog.run();
}
