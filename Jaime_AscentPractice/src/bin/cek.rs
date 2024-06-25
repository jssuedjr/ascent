use ascent::ascent;
use std::char;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::rc::Rc;
use std::ops::Deref;

// Environments map a variable to a closure
// Closure being (value + environment at time of assignment)
pub type Var = &'static char;
pub type Env = Rc<BTreeMap<Var, Closure>>;

// Enum defines possible states of an Expr type
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Expr {
    Symbol(Var),
	Var,
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

// Predicate function for expressions
/* 
fn is_expr(e: &Expr) -> bool {
    match e {
        Expr::Symbol(_) => true,
        Expr::Lambda(_param, body) => is_expr(body),
        Expr::App(lambda1, lambda2) => 
            is_expr(lambda1) && is_expr(lambda2),
        _ => false,
    }
}
*/

fn update(ρ: &Env, v: Var, cl: Closure) -> Rc<BTreeMap<Var, Closure>> {
    let mut ρ = ρ.deref().clone();
    ρ.insert(v, cl);
    Rc::new(ρ)
}

// CEK machine rules 
ascent! {
    struct CEK;
    // Defining CEK parameters
    relation ς(Expr, Env, Kont); // A set of sets (ordered by ⊆)
    relation input(Expr); // Define a rule: inj(e) <-- input(e);
    relation output(Expr); // Define a rule: output set of possibly reachable states for some terminating condition
    //relation eval();

    // Injection rule, create initial state with empty environment and empty continuation
    ς(e, Rc::new(BTreeMap::new()), Kont::Mt()) <-- input(e);

    // First rule in Fig. 1
    // If the environment ρ contains a Closure(Var, Closure)
    // substitute the value and the environment from the previous state into the new one.
    ς(v, ρ2, k) <-- 
        ς(?Expr::Symbol(x), ρ, k), 
        if let Some(Closure(v, ρ2)) = ρ.get(x);

    // Second rule in Fig. 1
    ς(e0, ρ.clone(), Kont::Ar(e1.deref().clone(), ρ.clone(), k.clone().into())) <--
        ς(?Expr::App(e0, e1), ρ, k);

    // Third rule in Fig. 1
    ς(e.clone(), ρ2.clone(), Kont::Fn(v.clone(), ρ.clone(), k2.clone())) <--
        ς(v, ρ, k),
        if let Kont::Ar(e, ρ2, k2) = k;
        //ς(v.clone(), ρ.clone(), Kont::Ar(e, ρ2, Arc::new(k)));

    // Fourth rule in Fig. 1
    ς(e.deref().clone(), new_ρ2, k2.deref().clone()) <--
        ς(v, ρ, k),
        if let Kont::Fn(a, ρ2, k2) = k,
        if let Expr::Lambda(x, e) = a,
        let new_ρ2 = update(&ρ2, *x, Closure(v.clone(), ρ.clone()));

    // when no more new states can be added, output the set of
    // reachable states
    // how can i write this rule
    output(e) <-- ς(e, _, Kont::Mt());
}

fn main() {
    //let x: Expr = Expr::Symbol('x');
    static X: char = 'x';
    let input: Expr = Expr::Lambda(&X, Arc::new(Expr::Symbol(&X)));
    //let input: Expr = Expr::Lambda(x, Arc::new(x.clone()));
    let mut prog: CEK = CEK::default();
    prog.input = vec![(input,)];
    prog.run();
    println!("reachable states: {:?}", prog.output);
}
