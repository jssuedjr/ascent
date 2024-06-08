use ascent::ascent;

// given a sequence of instructions, determine if the sequence is possible to execute based on the rules

ascent! {
    // instructions
    relation mov(&'static str, &'static str); 
    relation je(i32, &'static str); // if result of prior arithmetic operation is equal to zero, jump to address
    relation add(&'static str, &'static str);
    relation sub(&'static str, &'static str);

    // rules
    je(result, addr) <-- sub(r1, r2);
    mov(r2, r3) <-- add(r1, r2);
}

fn main() {
    let mut prog: AscentProgram = AscentProgram::default();

    prog.
    
    prog.run();

}