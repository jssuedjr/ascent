use ascent::ascent;

ascent! {
    relation friend(&'static str, &'static str);
    relation friend_of_friend(&'static str, &'static str);

    friend_of_friend(x, z) <-- friend(x, y), friend(y, z);
}

fn main() {
    let mut prog: AscentProgram = AscentProgram::default();

    prog.friend = vec![
        ("Steve", "Kris"),
        ("Kris", "Nick"),
        ("Mike", "Peter")
    ];
    
    prog.run();

    for (x, y) in prog.friend_of_friend.iter() {
        println!("{} is a friend of a friend of {}", x, y);
    }

}