use crate::number::NumberType;
use crate::position::Position;
use crate::symbols::SymbolMap;

// Context Or Scope?
#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    display_name: String,
    parent: Option<Box<Context>>,
    parent_pos: Option<Position>,
    symbol_map: Option<SymbolMap<NumberType>>,
}

impl Context {
    pub fn new(
        display_name: &str,
        parent: Option<Box<Context>>,
        parent_pos: Option<Position>,
        symbol_map: Option<SymbolMap<NumberType>>,
    ) -> Self {
        Self {
            display_name: display_name.to_string(),
            parent,
            parent_pos,
            symbol_map,
        }
    }

    pub fn init(display_name: &str) -> Self {
        // Assumes no parent. This is the root context
        Self {
            display_name: display_name.to_string(),
            parent: None,
            parent_pos: None,
            symbol_map: None,
        }
    }

    pub fn display_name(&self) -> String {
        self.display_name.clone()
    }

    pub fn parent(&self) -> Option<Box<Context>> {
        self.parent.clone()
    }

    pub fn parent_pos(&self) -> Option<Position> {
        self.parent_pos.clone()
    }

    pub fn symbol_map(&self) -> Option<SymbolMap<NumberType>> {
        self.symbol_map.clone()
    }

    pub fn set_symbol_map(&mut self, symbol_map: SymbolMap<NumberType>) {
        self.symbol_map = Some(symbol_map);
    }
}
