use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol
{
    pub value: usize,
}

pub struct SymbolTable
{
    pub counter: usize,
    pub table: HashMap<String, usize>,
    pub strings: Vec<String>,
}

impl SymbolTable
{
    pub fn new() -> SymbolTable
    {
        SymbolTable { counter: 0, table: HashMap::new(), strings: Vec::new() }
    }

    pub fn get_symbol(&mut self, s: &str) -> Symbol
    {
        if let Some(&value) = self.table.get(s)
        {
            Symbol { value }
        }
        else
        {
            let value = self.counter;
            self.counter += 1;
            self.table.insert(s.to_owned(), value);
            self.strings.push(s.to_owned());
            Symbol { value }
        }
    }

    pub fn try_get_symbol(&self, s: &str) -> Option<Symbol>
    {
        if let Some (&value) = self.table.get(s)
        {
            Some(Symbol { value })
        }
        else
        {
            None
        }
    }

    pub fn get_string(&self, symbol: Symbol) -> &str
    {
        &self.strings[symbol.value]
    }
}
