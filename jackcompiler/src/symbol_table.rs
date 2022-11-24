use std::collections::HashMap;
use crate::parser::Name;

pub type Index = usize;

#[derive(Debug)]
pub struct SymbolTable {
    table: HashMap<Name, SymbolInfo>,
    var_idx: HashMap<VarKind, Index>,
}

#[derive(Debug)]
struct SymbolInfo {
    var_type: VarType,
    var_kind: VarKind,
    index: Index,
}

#[derive(Debug)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum VarKind {
    Local,
    Argument,
    Static,
    This
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum VarType {
    Char, 
    Integer,
    Boolean,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let table = HashMap::new();
        let var_idx = HashMap::new();
        SymbolTable { table, var_idx }
    }

    pub fn define(&mut self, name: Name, var_kind: VarKind, var_type: VarType) {
        let index = self.var_count(var_kind);
        self.table.insert(name, SymbolInfo { var_type, var_kind, index });
    } 

    fn var_count(&mut self, var_kind: VarKind) -> Index {
        if self.var_idx.contains_key(&var_kind) {
            let new_value = self.var_idx.get(&var_kind).expect("Checked that key exists") + 1;
            self.var_idx.insert(var_kind, new_value);
            new_value
        } else {
            self.var_idx.insert(var_kind, 0);
            0 as Index
        }
    }

    pub fn reset(&mut self) -> &mut Self {
        self.table.clear();
        self.var_idx.clear();
        self
    }

    pub fn kind_of(&self, token: &Name) -> Option<VarKind> {
        self.table.get(token).map(|i| {
            i.var_kind
        })
    }

    pub fn type_of(&self, token: &Name) -> Option<VarType> {
        self.table.get(token).map(|i| {
            i.var_type
        })
    }

    pub fn index_of(&self, token: &Name) -> Option<Index> {
        self.table.get(token).map(|i| {
            i.index
        })
    }
}