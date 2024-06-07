use ascent::ascent;

ascent! {
    relation foo(i32); 
    
    //populating relation
    foo(x) <-- for x in 0..5;
}

fn main() {
    let mut prog: AscentProgram = AscentProgram::default();
    prog.run();
    println!("foo: {:?}", prog.foo);
}
