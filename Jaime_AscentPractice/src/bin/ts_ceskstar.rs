// implementation of Time-stamped CESK* machine from Figure 4 of AAM Paper
// featuring cleaner code! 
// attempt 0-cfa analysis 

use ascent::ascent;
use std::char;
use std::collections::BTreeMap; // good for ordered collections. hashmaps are unordered
//use std::sync::Arc;
use std::rc::Rc;
use std::ops::Deref;
use rand::Rng;

// VARIABLES AND ENVIRONMENT
pub type Time = u32; 
pub type Addr = u32;
pub type Var = &'static char;
pub type Env = Rc< BTreeMap<&'static char, Addr> >;

// EXPRESSIONS
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Expr {
	Sym(Var), //symbol
    Lambda(Var, Rc<Expr>),
    App(Rc<Expr>, Rc<Expr>)
}

// CLOSURES
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Closure(Expr, Env);

// CONTINUATIONS
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Kont {
    Fn(Expr, Env, Rc<Addr>),
    Ar(Expr, Env, Rc<Addr>),
    Mt()
}

// STORE
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Storable {
    Value(Closure),
    Continuation(Kont),
}

// updates the existing environment
fn update(environment: &Env, v: Var, addr: Addr) -> Env {
    let mut environment: BTreeMap<&char, Addr> = environment.deref().clone();
    environment.insert(v, addr);
    Rc::new(environment)
}

fn tick(_t: Time) -> Time {
    return 0;
}

#[allow(dead_code)]
fn alloc(_t: Time) -> Time {
    return 0;
}

fn gen_addr() -> Rc<u32> {
    let mut rng = rand::thread_rng();
    Rc::new(rng.gen_range(0..100))}

// time-stamped CESK* machine rules 
ascent! {
    struct CEK;
    relation state(Expr, Env, Rc<Addr>, Time); 
    relation store(Rc<Addr>, Storable); // the store will be a set of tuples! 
    relation input(Expr); 
    relation output(Expr, Env, Rc<Addr>, Time);

    // Injection rule, create initial state with empty environment and empty continuation
    state(e.clone(), 
        Rc::new(BTreeMap::new()), Rc::clone(&addr), 0),
    store(Rc::clone(&addr), Storable::Continuation(Kont::Mt()))
        <-- 
    input(e),
    let addr = gen_addr();

    // First rule in Fig. 4
    state(v, envir2, k, tick(*t)), 
    output(Expr::Sym(x.clone()), Rc::clone(&envir), k, t) 
        <-- 
    state(?Expr::Sym(x), envir, k, t),
    if let Some(a@Addr) = envir.get(x),
    //let a = envir.get(x),
    store(Rc::new(*a), ?Storable::Value(Closure(v, envir2)));

    // Second rule in Fig. 4
    state(e0, Rc::clone(&envir), Rc::clone(&b), tick(*t)), 
    store(Rc::clone(&b), Storable::Continuation(Kont::Ar(e0.deref().clone(), Rc::clone(&envir), Rc::clone(&a)))),
    output(Expr::App(e0.clone(), e1.clone()), Rc::clone(&envir), a, t) 
        <--
    state(?Expr::App(e0, e1), envir, a, t),
    let b = gen_addr();

    // Third rule in Fig. 4
    state(e, Rc::clone(&envir), Rc::clone(&b), tick(*t)),
    store(Rc::clone(&b), Storable::Continuation(Kont::Fn(v.clone(), Rc::clone(&envir), Rc::clone(&c)))),
    output(v, Rc::clone(&envir), a, t) 
        <--
    state(v, envir, a, t),
    store(a, ?Storable::Continuation(Kont::Ar(e, envir_copy, c))),
    let b = gen_addr();

    // Fourth rule in Fig. 4
    state(e, Rc::clone(&new_envir2), c, tick(*t)), 
    store(b, Storable::Value(Closure(v.clone(), Rc::clone(&envir)))),
    output(v, envir, a, t) 
        <--
    state(v, envir, a, t),
    store(a, ?Storable::Continuation(Kont::Fn(lambda, envir2, c))),
    if let Expr::Lambda(x, e) = lambda,
    let b = gen_addr(),
    let new_envir2 = update(&envir2, *x, *b);

    // When no more new states can be added, output the set of reachable states
    // CHANGE THIS RULE TO CORRECTLY WORK, KEEP ADDING NEW STATES TO OUTPUT
    // the way i have it now will only add states to output if it matches the pattern, so
    // other states will be ignored that should be in there
    output(e, envir, a, t) 
        <-- 
    state(e, envir, a, t),
    if let Expr::Lambda(_, _) = e,
    store(a, Storable::Continuation(Kont::Mt()));
}

fn main() {
    let x: Var = &'x';
    let y: Var = &'y';
    let input: Expr = 
        Expr::App
        (Rc::new(Expr::Lambda(x, Rc::new(Expr::Sym(x)))), 
        Rc::new(Expr::Lambda(y, Rc::new(Expr::Sym(y)))));

    // u combinator
    /* let input: Expr = 
        Expr::App
        (Rc::new(Expr::Lambda(x, Rc::new(Expr::App(Rc::new(Expr::Sym(x)), Rc::new(Expr::Sym(x)))))), 
        Rc::new(Expr::Lambda(y, Rc::new(Expr::App(Rc::new(Expr::Sym(y)), Rc::new(Expr::Sym(y)))))));
    */

    let mut prog: CEK = CEK::default();
    prog.input = vec![(input,)];
    prog.run();
    println!("States:\n");
    println!("Input expression: {:?}", prog.input);
    for item in prog.output.iter() {
        println!("Next state: {:?}", item);
    }

    println!("Store:");
    for item in prog.store.iter() {
        println!("Next storable: {:?}", item);
    }
}
