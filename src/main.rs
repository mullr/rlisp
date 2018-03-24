use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct LSymbol {
    name: String,
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
    match (scope.bindings.get(sym), &scope.parent) {
        (Some(val), ..) => val.clone(),
        (None, &Some(ref parent_scope)) => resolve_symbol(parent_scope, sym),
        (None, &None) => LValue::Nil,
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
        name: name.to_string(),
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

    println!("x: {:?}", eval(&LValue::Symbol{sym:x}, &scope));
    println!("pi: {:?}", eval(&LValue::Symbol{sym:pi}, &scope));
}
