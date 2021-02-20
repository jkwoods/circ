use crate::ir::term::*;

use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter, Debug};
use std::rc::Rc;
use thiserror::Error;
use log::debug;

pub mod includer;
pub mod mem;

type Version = usize;

type VarName = String;

type SsaName = String;

/// A *value*. Can be a term, or a reference.
#[derive(Clone, Debug)]
pub enum Val<T> {
    Term(T),
    Ref(Loc),
}

#[derive(Error, Debug)]
pub enum CircError {
    #[error("Location '{0}' is invalid")]
    InvalidLoc(Loc),
    #[error("Identifier '{0}' cannot be found")]
    NoName(VarName),
    #[error("No lexical scope in fn '{0}'")]
    NoScope(String),
    #[error("Identifier '{0}' already declared as a '{1}'")]
    Rebind(String, String),
    #[error("Term '{0}' is not a boolean, as conditionals must be.")]
    NotBool(Term),
    #[error("The break label '{0}' is unknown.")]
    UnknownBreak(String),
    #[error("Cannot assign '{0}' to location '{1}', which held '{2}'")]
    MisTypedAssign(String, String, String),
    #[error("Function '{0}' has a return value: {1}. Return stmt has a value: {2}'")]
    ReturnMismatch(String, bool, bool)
}

pub type Result<T> = std::result::Result<T, CircError>;

impl<T: Display> Display for Val<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Val::Term(t) => write!(f, "{}", t),
            Val::Ref(l) => write!(f, "&{}", l),
        }
    }
}

impl<T> Val<T> {
    pub fn unwrap_term(self) -> T {
        match self {
            Val::Term(t) => t,
            Val::Ref(l) => panic!("{:?} is not a term", l),
        }
    }
}

/// A *location*. Can be a name in the current scope, or a reference to an alternate scope.
#[derive(Debug, Clone)]
pub struct Loc {
    name: VarName,
    /// Optional (fn, lex) scope indices
    /// The outer option indicates whether this is a local or absolute location.
    /// The inner indicates whether this is a global variable.
    idx: Option<Option<ScopeIdx>>,
}

impl Loc {
    pub fn local(name: VarName) -> Self {
        Self { name, idx: None }
    }
}

impl Display for Loc {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.idx {
            Some(_) => write!(f, "*{}", self.name),
            None => write!(f, "{}", self.name),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ScopeIdx {
    fn_: usize,
    lex: usize,
}

#[derive(Hash, PartialEq, Eq)]
struct VerVar {
    name: VarName,
    ver: Version,
}

#[derive(Debug)]
struct LexEntry<Ty> {
    ver: Version,
    ssa_name: SsaName,
    name: String,
    ty: Ty,
}

impl<Ty> LexEntry<Ty> {
    pub fn next_ver(&mut self) {
        self.ver += 1;
        self.set_ssa_name();
    }
    fn set_ssa_name(&mut self) {
        self.ssa_name = format!("{}_v{}", self.name, self.ver);
    }
    pub fn new(name: String, ty: Ty) -> Self {
        let mut this = Self {
            ver: 0,
            ssa_name: "".to_string(),
            name,
            ty,
        };
        this.set_ssa_name();
        this
    }
}

#[derive(Debug)]
struct LexScope<Ty> {
    entries: HashMap<VarName, LexEntry<Ty>>,
    prefix: String,
}

impl<Ty: Display> LexScope<Ty> {
    fn with_prefix(prefix: String) -> Self {
        Self {
            prefix,
            entries: HashMap::new(),
        }
    }
    fn declare(&mut self, name: VarName, ty: Ty) -> Result<&SsaName> {
        let p = &self.prefix;
        match self.entries.entry(name.clone()) {
            Entry::Vacant(v) => Ok(&v
                .insert(LexEntry::new(format!("{}_{}", p, name), ty))
                .ssa_name),
            Entry::Occupied(o) => Err(CircError::Rebind(name, format!("{}", o.get().ty))),
        }
    }
    fn get_name(&self, name: &str) -> Result<&SsaName> {
        Ok(&self.entries.get(name).ok_or_else(|| CircError::NoName(name.to_owned()))?.ssa_name)
    }
    fn get_ty(&self, name: &str) -> Result<&Ty> {
        Ok(&self.entries.get(name).ok_or_else(|| CircError::NoName(name.to_owned()))?.ty)
    }
    fn next_ver(&mut self, name: &str) -> Result<&SsaName> {
        let e = self.entries.get_mut(name).ok_or_else(|| CircError::NoName(name.to_owned()))?;
        e.next_ver();
        Ok(&e.ssa_name)
    }
    fn has_name(&self, name: &str) -> bool {
        self.entries.contains_key(name)
    }
}

pub struct CirCtx {
    pub mem: mem::MemManager,
    pub cs: Rc<RefCell<Constraints>>,
}

#[derive(Debug)]
enum StateEntry<Ty> {
    Lex(LexScope<Ty>),
    Cond(Term),
    /// A block `name`, which has been broken out of if any condition is satisfied.
    Break(String, Vec<Term>),
}

#[derive(Debug)]
pub struct FnFrame<Ty> {
    stack: Vec<StateEntry<Ty>>,
    scope_ctr: usize,
    prefix: String,
    name: String,
    has_return: bool,
}

impl<Ty: Display> FnFrame<Ty> {
    fn new(name: String, prefix: String, has_return: bool) -> Self {
        let mut this = Self {
            stack: Vec::new(),
            scope_ctr: 0,
            prefix,
            name,
            has_return,
        };
        this.enter_scope();
        this.enter_breakable(RET_BREAK_NAME.to_owned());
        this
    }
    fn last_lex_mut(&mut self) -> Result<&mut LexScope<Ty>> {
        if let Some(n) = self
            .stack
            .iter_mut()
            .rev()
            .filter_map(|f| match f {
                StateEntry::Lex(l) => Some(l),
                _ => None,
            })
            .next()
        {
            Ok(n)
        } else {
            Err(CircError::NoScope(self.name.clone()))
        }
    }
    pub fn declare(&mut self, name: VarName, ty: Ty) -> Result<&SsaName> {
        self.last_lex_mut()?.declare(name, ty)
    }
    pub fn enter_scope(&mut self) {
        self.stack
            .push(StateEntry::Lex(LexScope::with_prefix(format!(
                "{}_lex{}",
                self.prefix, self.scope_ctr
            ))));
        self.scope_ctr += 1;
    }
    #[track_caller]
    pub fn exit_scope(&mut self) {
        if let Some(StateEntry::Lex(_)) = self.stack.pop() {
        } else {
            panic!("Stack does not end with a scope");
        }
    }
    fn enter_condition(&mut self, condition: Term) -> Result<()> {
        if check(&condition) == Sort::Bool {
            Ok(self.stack.push(StateEntry::Cond(condition)))
        } else {
            Err(CircError::NotBool(condition))
        }
    }
    #[track_caller]
    fn exit_condition(&mut self) {
        if let Some(StateEntry::Cond(_)) = self.stack.pop() {
        } else {
            panic!("Stack does not end with a condition");
        }
    }
    fn conditions(&self) -> Vec<Term> {
        let mut cs = Vec::new();
        for s in &self.stack {
            match s {
                StateEntry::Cond(t) => cs.push(t.clone()),
                StateEntry::Break(_, break_conds) => {
                    cs.extend(break_conds.iter().map(|c| term![Op::Not; c.clone()]))
                }
                _ => {}
            }
        }
        cs
    }

    fn enter_breakable(&mut self, name: String) {
        self.stack.push(StateEntry::Break(name, Vec::new()));
    }

    #[track_caller]
    fn exit_breakable(&mut self) {
        if let Some(StateEntry::Break(..)) = self.stack.pop() {
        } else {
            panic!("Stack does not end with a breakable block");
        }
    }

    fn break_(&mut self, name: &str) -> Result<()> {
        // Walk back to the breakable block, accumulating conditions...
        let mut break_if = Vec::new();
        for s in self.stack.iter_mut().rev() {
            match s {
                StateEntry::Cond(c) => break_if.push(c.clone()),
                StateEntry::Break(name_, ref mut break_conds) => {
                    if name_ == name {
                        break_conds.push(if break_if.len() == 0 {
                            leaf_term(Op::Const(Value::Bool(true)))
                        } else {
                            term(Op::BoolNaryOp(BoolNaryOp::And), break_if)
                        });
                        return Ok(());
                    } else {
                        break_if.extend(break_conds.iter().map(|c| term![Op::Not; c.clone()]));
                    }
                }
                _ => {}
            }
        }
        Err(CircError::UnknownBreak(name.to_owned()))
    }
}

/// A language whose state can be managed
pub trait Embeddable {
    /// Type for this language
    type Ty: Display + Clone + Debug;
    /// Terms for this language
    type T: Display + Clone + Debug;

    fn declare(
        &self,
        ctx: &mut CirCtx,
        ty: &Self::Ty,
        raw_name: String,
        user_name: Option<String>,
        public: bool,
    ) -> Self::T;
    fn ite(&self, ctx: &mut CirCtx, cond: Term, t: Self::T, f: Self::T) -> Self::T;
    fn assign(&self, ctx: &mut CirCtx, ty: &Self::Ty, name: String, t: Self::T) -> Self::T;
    fn values(&self) -> bool;
}

pub struct Circify<E: Embeddable> {
    e: E,
    vals: HashMap<String, Val<E::T>>,
    fn_stack: Vec<FnFrame<E::Ty>>,
    fn_ctr: usize,
    globals: LexScope<E::Ty>,
    cir_ctx: CirCtx,
    condition: Term,
    typedefs: HashMap<String, E::Ty>,
}

impl<E: Embeddable> Debug for Circify<E> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Circify")
            .field("vals", &self.vals)
            .field("fn_stack", &self.fn_stack)
            .field("globals", &self.globals)
            .field("cir_ctx", &"*omitted*")
            .field("condition", &self.condition)
            .field("typedefs", &self.typedefs)
            .finish()
    }
}

impl<E: Embeddable> Circify<E> {
    pub fn new(e: E) -> Self {
        let cs = Rc::new(RefCell::new(Constraints::new(e.values())));
        Self {
            e,
            vals: HashMap::new(),
            fn_stack: Vec::new(),
            fn_ctr: 0,
            globals: LexScope::with_prefix("global".to_string()),
            cir_ctx: CirCtx {
                mem: mem::MemManager::new(cs.clone()),
                cs,
            },
            condition: leaf_term(Op::Const(Value::Bool(true))),
            typedefs: HashMap::new(),
        }
    }

    /// Declares a new (unconstrained) value of type `ty`, with name `name`, in the current lexical
    /// scope.
    /// If `input`, then the language's declaration function is provided with `name` as the
    /// user-visible name for this variable, which can be used to look up user-provided values.
    /// If `public`, then the language's declaration function is told to make this variable public.
    pub fn declare(&mut self, name: VarName, ty: &E::Ty, input: bool, public: bool) -> Result<()> {
        let ssa_name = if let Some(back) = self.fn_stack.last_mut() {
            back.declare(name.clone(), ty.clone())?
        } else {
            self.globals.declare(name.clone(), ty.clone())?
        };
        let t = self.e.declare(
            &mut self.cir_ctx,
            ty,
            ssa_name.to_string(),
            if input { Some(name) } else { None },
            public,
        );
        assert!(self.vals.insert(ssa_name.to_string(), Val::Term(t)).is_none());
        Ok(())
    }

    /// Lookup this name in the current fn/lexical scopes.
    /// Returns `None` if it's in the global scope.
    /// Returns `Some` if it's in a local scope.
    /// Errors if the name cannot be found.
    fn mk_abs(&self, name: &VarName) -> Result<Option<ScopeIdx>> {
        if let Some(fn_) = self.fn_stack.last() {
            for (lex_i, e) in fn_.stack.iter().enumerate().rev() {
                match e {
                    StateEntry::Lex(l) => {
                        if l.has_name(name) {
                            return Ok(Some(ScopeIdx {
                                lex: lex_i,
                                fn_: self.fn_stack.len() - 1,
                            }));
                        }
                    }
                    _ => {}
                }
            }
        }
        if self.globals.has_name(name) {
            Ok(None)
        } else {
            Err(CircError::NoName(name.clone()))
        }
    }

    /// Gets the indicated scope.
    /// A `None` scope is the global one.
    fn get_scope_mut(&mut self, idx: Option<ScopeIdx>) -> &mut LexScope<E::Ty> {
        match idx {
            None => &mut self.globals,
            Some(ScopeIdx { lex, fn_ }) => {
                if let StateEntry::Lex(e) = &mut self.fn_stack[fn_].stack[lex] {
                    e
                } else {
                    unreachable!()
                }
            }
        }
    }

    /// Gets the indicated scope.
    /// A `None` scope is the global one.
    fn get_scope_ref(&self, idx: Option<ScopeIdx>) -> &LexScope<E::Ty> {
        match idx {
            None => &self.globals,
            Some(ScopeIdx { lex, fn_ }) => {
                if let StateEntry::Lex(e) = &self.fn_stack[fn_].stack[lex] {
                    e
                } else {
                    unreachable!()
                }
            }
        }
    }

    fn get_lex_mut(&mut self, loc: &Loc) -> Result<&mut LexScope<E::Ty>> {
        let idx = loc.idx.ok_or(()).or_else(|_| self.mk_abs(&loc.name))?;
        Ok(self.get_scope_mut(idx))
    }

    fn get_lex_ref(&self, loc: &Loc) -> Result<&LexScope<E::Ty>> {
        let idx = loc.idx.ok_or(()).or_else(|_| self.mk_abs(&loc.name))?;
        Ok(self.get_scope_ref(idx))
    }

    pub fn declare_init(
        &mut self,
        name: VarName,
        ty: E::Ty,
        val: Val<E::T>,
        public: bool,
    ) -> Result<Val<E::T>> {
        self.declare(name.clone(), &ty, false, public)?;
        self.assign(Loc::local(name), val)
    }

    pub fn assign(&mut self, loc: Loc, val: Val<E::T>) -> Result<Val<E::T>> {
        let lex = self.get_lex_mut(&loc)?;
        let old_name = lex.get_name(&loc.name)?.clone();
        let ty = lex.get_ty(&loc.name)?.clone();
        let new_name = lex.next_ver(&loc.name)?.clone();
        let old_val = self.vals.get(&old_name).unwrap();
        match (old_val, val) {
            (Val::Term(old), Val::Term(new)) => {
                let guard = self.condition.clone();
                let ite_val = self.e.ite(&mut self.cir_ctx, guard, (*old).clone(), new);
                let new_val =
                    Val::Term(
                        self.e
                            .assign(&mut self.cir_ctx, &ty, new_name.clone(), ite_val),
                    );
                assert!(self.vals.insert(new_name, new_val.clone()).is_none());
                Ok(new_val)
            }
            (_, v @ Val::Ref(_)) => {
                // TODO: think about this more...
                self.vals.insert(new_name, v.clone());
                Ok(v)
            }
            (_, v) => Err(CircError::MisTypedAssign(
                format!("{}", v),
                format!("{}", loc),
                format!("{}", old_val),
            )),
        }
    }

    pub fn enter_breakable(&mut self, name: String) {
        self.fn_stack
            .last_mut()
            .expect("No fn")
            .enter_breakable(name);
    }

    #[track_caller]
    pub fn exit_breakable(&mut self) {
        self.fn_stack.last_mut().expect("No fn").exit_breakable();
    }

    #[track_caller]
    pub fn break_(&mut self, name: &str) -> Result<()> {
        self.fn_stack.last_mut().expect("No fn").break_(name)
    }

    #[track_caller]
    pub fn enter_scope(&mut self) {
        self.fn_stack.last_mut().expect("No fn").enter_scope()
    }

    #[track_caller]
    pub fn exit_scope(&mut self) {
        self.fn_stack.last_mut().expect("No fn").exit_scope()
    }

    pub fn enter_condition(&mut self, condition: Term) -> Result<()> {
        assert!(check(&condition) == Sort::Bool);
        self.fn_stack
            .last_mut()
            .expect("No fn")
            .enter_condition(condition)?;
        self.condition = self.condition();
        Ok(())
    }

    pub fn exit_codition(&mut self) {
        self.fn_stack.last_mut().expect("No fn").exit_condition();
        self.condition = self.condition();
    }

    pub fn condition(&self) -> Term {
        // TODO:  more precise conditions, depending on lex scopes.
        let cs: Vec<_> = self.fn_stack.iter().flat_map(|f| f.conditions()).collect();
        if cs.len() == 0 {
            leaf_term(Op::Const(Value::Bool(true)))
        } else {
            term(Op::BoolNaryOp(BoolNaryOp::And), cs)
        }
    }

    pub fn enter_fn(&mut self, name: String, ret_ty: Option<E::Ty>) {
        let prefix = format!("{}_f{}", name, self.fn_ctr);
        self.fn_ctr += 1;
        self.fn_stack.push(FnFrame::new(name, prefix, ret_ty.is_some()));
        if let Some(ty) = ret_ty {
            self.declare(RET_NAME.to_owned(), &ty, false, false).expect("Bad ret name in fn enter");
        }
    }

    pub fn return_(&mut self, val: Option<E::T>) -> Result<()> {
        let last = self.fn_stack.last().expect("No fn");
        if val.is_some() != last.has_return {
            return Err(CircError::ReturnMismatch(last.name.clone(), last.has_return, val.is_some()));
        }
        if let Some(r) = val {
            self.assign(
                Loc {
                    name: RET_NAME.to_owned(),
                    idx: None,
                },
                Val::Term(r),
            )?;
        }
        self.break_(RET_BREAK_NAME)
    }

    pub fn assert(&mut self, t: Term) {
        self.cir_ctx.cs.borrow_mut().assert(t);
    }

    #[track_caller]
    pub fn exit_fn(&mut self) -> Option<Val<E::T>> {
        if let Some(fn_) = self.fn_stack.last() {
            let ret = if fn_.has_return {
                Some(self.get_value(Loc::local(RET_NAME.to_owned())).unwrap())
            } else {
                None
            };
            self.fn_stack.pop().unwrap();
            ret
        } else {
            panic!("No fn to exit")
        }
    }

    pub fn get_value(&self, loc: Loc) -> Result<Val<E::T>> {
        let l = self.get_lex_ref(&loc)?;
        self.vals
            .get(l.get_name(&loc.name)?)
            .cloned()
            .ok_or_else(|| CircError::InvalidLoc(loc))
    }

    pub fn deref(&self, v: &Val<E::T>) -> Loc {
        match v {
            Val::Ref(l) => l.clone(),
            Val::Term(_) => panic!("{} is not dereferencable", v),
        }
    }
    pub fn ref_(&self, name: &VarName) -> Result<Val<E::T>> {
        let idx = self.mk_abs(name)?;
        Ok(Val::Ref(Loc {
            name: name.to_owned(),
            idx: Some(idx),
        }))
    }
    pub fn def_type(&mut self, name: &str, ty: E::Ty) {
        if let Some(old_ty) = self.typedefs.insert(name.to_owned(), ty) {
            panic!("{} already defined as {}", name, old_ty)
        }
    }
    pub fn get_type(&mut self, name: &str) -> &E::Ty {
        self.typedefs
            .get(name)
            .unwrap_or_else(|| panic!("No type {}", name))
    }
    pub fn consume(self) -> Rc<RefCell<Constraints>> {
        self.cir_ctx.cs
    }
}

const RET_NAME: &str = "return";
const RET_BREAK_NAME: &str = "return";

#[cfg(test)]
mod test {
    use super::*;

    mod bool_pair {
        use super::*;

        #[derive(Clone, Debug)]
        enum T {
            Base(Term),
            Pair(Box<T>, Box<T>),
        }

        impl Display for T {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                match self {
                    T::Base(t) => write!(f, "{}", t),
                    T::Pair(a, b) => write!(f, "({}, {})", a, b),
                }
            }
        }

        #[derive(Clone, Debug)]
        enum Ty {
            Bool,
            Pair(Box<Ty>, Box<Ty>),
        }

        impl Display for Ty {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                match self {
                    Ty::Bool => write!(f, "bool"),
                    Ty::Pair(a, b) => write!(f, "({}, {})", a, b),
                }
            }
        }

        #[derive(Clone)]
        enum Val {
            Base(bool),
            Pair(Box<T>, Box<T>),
        }

        struct BoolPair {
            values: Option<HashMap<String, bool>>,
        }

        impl Embeddable for BoolPair {
            type T = T;
            type Ty = Ty;
            fn declare(
                &self,
                ctx: &mut CirCtx,
                ty: &Self::Ty,
                raw_name: String,
                user_name: Option<String>,
                public: bool,
            ) -> Self::T {
                match ty {
                    Ty::Bool => {
                        if public {
                            ctx.cs.borrow_mut().public_inputs.insert(raw_name.clone());
                        }
                        T::Base(ctx.cs.borrow_mut().new_var(
                            &raw_name,
                            Sort::Bool,
                            || {
                                Value::Bool(
                                    user_name
                                        .as_ref()
                                        .and_then(|v| {
                                            self.values.as_ref().and_then(|vs| vs.get(v).cloned())
                                        })
                                        .unwrap_or(false),
                                )
                            },
                            user_name.is_some(),
                        ))
                    }
                    Ty::Pair(a, b) => T::Pair(
                        Box::new(self.declare(
                            ctx,
                            &**a,
                            format!("{}.0", raw_name),
                            user_name.as_ref().map(|u| format!("{}.0", u)),
                            public,
                        )),
                        Box::new(self.declare(
                            ctx,
                            &**b,
                            format!("{}.1", raw_name),
                            user_name.as_ref().map(|u| format!("{}.1", u)),
                            public,
                        )),
                    ),
                }
            }
            fn ite(&self, ctx: &mut CirCtx, cond: Term, t: Self::T, f: Self::T) -> Self::T {
                match (t, f) {
                    (T::Base(a), T::Base(b)) => T::Base(term![Op::Ite; cond, a, b]),
                    (T::Pair(a0, a1), T::Pair(b0, b1)) => T::Pair(
                        Box::new(self.ite(ctx, cond.clone(), *a0, *b0)),
                        Box::new(self.ite(ctx, cond, *a1, *b1)),
                    ),
                    (a, b) => panic!("Cannot ITE {}, {}", a, b),
                }
            }
            fn assign(
                &self,
                ctx: &mut CirCtx,
                _ty: &Self::Ty,
                name: String,
                t: Self::T,
            ) -> Self::T {
                match t {
                    T::Base(a) => {
                        ctx.cs.borrow_mut().eval_and_save(&name, &a);
                        let v = leaf_term(Op::Var(name, Sort::Bool));
                        ctx.cs
                            .borrow_mut()
                            .assertions
                            .push(term![Op::Eq; v.clone(), a]);
                        T::Base(v)
                    }
                    T::Pair(a, b) => T::Pair(
                        Box::new(self.assign(ctx, _ty, format!("{}.0", name), *a)),
                        Box::new(self.assign(ctx, _ty, format!("{}.1", name), *b)),
                    ),
                }
            }
            fn values(&self) -> bool {
                self.values.is_some()
            }
        }

        #[test]
        fn trial() {
            let values: HashMap<String, bool> = vec![
                ("a".to_owned(), false),
                ("b.0".to_owned(), false),
                ("b.1".to_owned(), false),
            ]
            .into_iter()
            .collect();
            let e = BoolPair {
                values: Some(values),
            };
            let mut c = Circify::new(e);
            c.declare("a".to_owned(), &Ty::Bool, true, false).unwrap();
            c.declare(
                "b".to_owned(),
                &Ty::Pair(Box::new(Ty::Bool), Box::new(Ty::Bool)),
                true,
                false,
            ).unwrap();
        }
    }
}