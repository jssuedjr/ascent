//Better implementation of CEK

use ascent::ascent;
use std::char;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::rc::Rc;
use std::ops::Deref;

// Environments map a variable to a closure
// Closure being (value + environment at time of assignment)
pub type Var = &'static char;
pub type Env = Rc<BTreeMap<&'static char, Closure>>;

// Enum defines possible states of an Expr type
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Expr {
	Sym(Var), //symbol
    Lambda(Var, Arc<Expr>),
    App(Arc<Expr>, Arc<Expr>)
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Closure(Expr, Env);

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Kont {
    Fn(Expr, Env, Arc<Kont>),
    Ar(Expr, Env, Arc<Kont>),
    Mt()
}

// updates the existing environment
fn update(environment: &Env, v: Var, cl: Closure) -> Rc<BTreeMap<Var, Closure>> {
    let mut environment = environment.deref().clone();
    environment.insert(v, cl);
    Rc::new(environment)
}

// CEK machine rules 
// need to make it so that output is a collection of reachable possible states
ascent! {
    struct CEK;
    // Defining CEK parameters
    relation state(Expr, Env, Kont); // A set of sets (ordered by ⊆)
    //relation environment(Var, Env); // in the environment, it binds variables to closures
    relation input(Expr); // Define a rule: inj(e) <-- input(e);
    relation output(Expr, Env, Kont); // Define a rule: output set of possibly reachable states for some terminating condition

    // Injection rule, create initial state with empty environment and empty continuation
    state(e.clone(), Rc::new(BTreeMap::new()), Kont::Mt()) <-- input(e);

    // First rule in Fig. 1
    // If the environment ρ contains a Closure(Var, Closure)
    // substitute the value and the environment from the previous state into the new one.
    state(v, envir2, k), output(Expr::Sym(x.clone()), envir.clone(), k.clone()) <-- 
        state(?Expr::Sym(x), envir, k), 
        if let Some(Closure(v, envir2)) = envir.get(x);

    // Second rule in Fig. 1
    state(e0, envir.clone(), Kont::Ar(e1.deref().clone(), envir.clone(), k.clone().into())), output(Expr::App(e0.clone(), e1.clone()), envir.clone(), k.clone()) <--
        state(?Expr::App(e0, e1), envir, k);

    // Third rule in Fig. 1
    state(e.clone(), envir2.clone(), Kont::Fn(v.clone(), envir.clone(), k2.clone())), output (v.clone(), envir.clone(), k.clone()) <--
        state(v, envir, k),
        if let Kont::Ar(e, envir2, k2) = k;

    // Fourth rule in Fig. 1
    state(e.deref().clone(), new_envir2, k2.deref().clone()), output(v.clone(), envir.clone(), k.clone()) <--
        state(v, envir, k),
        if let Kont::Fn(a, envir2, k2) = k,
        if let Expr::Lambda(x, e) = a,
        let new_envir2 = update(&envir2, *x, Closure(v.clone(), envir.clone()));

    // When no more new states can be added, output the set of reachable states
    // CHANGE THIS RULE TO CORRECTLY WORK, KEEP ADDING NEW STATES TO OUTPUT
    // the way i have it now will only add states to output if it matches the pattern, so
    // other states will be ignored that should be in there
    output(e, envir, Kont::Mt()) <-- 
        state(e, envir, Kont::Mt()),
        if let Expr::Lambda(_, _) = e;
}

fn main() {
    static X: char = 'x';
    static Y: char = 'y';
    //let input: Expr = Expr::Lambda(&X, Arc::new(Expr::Sym(&X)));
    let input: Expr = Expr::App(Expr::Lambda(&X, Arc::new(Expr::Sym(&X))).into(), Expr::Lambda(&Y, Arc::new(Expr::Sym(&Y))).into());
    let mut prog: CEK = CEK::default();
    prog.input = vec![(input,)];
    prog.run();
    println!("Input expression: {:?}", prog.input);
    for item in prog.output.iter() {
        println!("Next state: {:?}", item);
    }
    //println!("reachable states: {:?}", prog.output);
}
