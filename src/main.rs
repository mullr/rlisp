use std::collections::HashMap;

extern crate string_cache;
use string_cache::DefaultAtom as Atom;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct LSymbol {
    atom: Atom,
}

#[derive(Hash, Clone, Debug)]
#[allow(dead_code)]
enum LValue {
    Nil,
    Integer { value: i64 },
    String { value: String },
    Symbol { sym: LSymbol },
    Cons { car: Box<LValue>, cdr: Box<LValue> },
}

struct Scope {
    parent: Option<Box<Scope>>,
    bindings: HashMap<LSymbol, LValue>,
}

fn resolve_symbol(scope: &Scope, sym: &LSymbol) -> LValue {
    let mut current_scope = scope;
    loop {
        match (current_scope.bindings.get(sym), &scope.parent) {
            (Some(val), ..) => return val.clone(),
            (None, &Some(ref parent_scope)) => current_scope = parent_scope,
            (None, &None) => return LValue::Nil,
        };
    }
}


fn eval(val: &LValue, scope: &Scope) -> LValue {
    match val {
        &LValue::Nil => val.clone(),
        &LValue::Integer { .. } => val.clone(),
        &LValue::String { .. } => val.clone(),
        &LValue::Symbol { ref sym } => resolve_symbol(scope, sym),
        &LValue::Cons { .. } => val.clone(),
    }
}

fn sym(name: &str) -> LSymbol {
    LSymbol {
        atom: Atom::from(name),
    }
}

fn main() {
    // let x = LValue::Cons{ car: Box::new(LValue::Symbol{ name: "+".to_string()}),
    //                       cdr: Box::new(LValue::Nil) };

    let x = sym("x");
    let pi = sym("pi");


    let mut root_bindings = HashMap::new();
    root_bindings.insert(pi.clone(), LValue::Integer { value: 3 });

    let root_scope = Scope {
        parent: None,
        bindings: root_bindings,
    };


    let mut bindings = HashMap::new();
    bindings.insert(x.clone(), LValue::Integer { value: 42 });


    let scope = Scope {
        parent: Some(Box::new(root_scope)),
        bindings: bindings,
    };

    println!("x sym:: {:?}", x);
    println!("x: {:?}", eval(&LValue::Symbol{sym:x}, &scope));
    println!("pi: {:?}", eval(&LValue::Symbol{sym:pi}, &scope));

    // println!("x: {:?}", Atom::from("foo"));

}
