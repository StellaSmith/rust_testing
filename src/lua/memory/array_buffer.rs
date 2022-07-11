#[derive(Debug, Clone)]
pub struct ArrayBuffer {
    _slice: std::rc::Rc<[u8]>,
}

impl ArrayBuffer {
    pub fn new(size: usize) -> Result<Self, std::collections::TryReserveError> {
        let mut v = Vec::new();
        v.try_reserve_exact(size)?;
        v.resize(size, 0);
        Ok(Self { _slice: v.into() })
    }

    pub fn slice(&self) -> &[u8] {
        &self._slice
    }

    pub fn len(&self) -> usize {
        self._slice.len()
    }
}

impl Default for ArrayBuffer {
    fn default() -> Self {
        const ARRAY: [u8; 0] = [];
        Self {
            _slice: std::rc::Rc::from(ARRAY.as_slice()),
        }
    }
}

impl std::fmt::Display for ArrayBuffer {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str("ArrayBuffer { ")?;
        use std::ops::Deref;
        for v in self._slice.deref() {
            write!(fmt, "{}, ", v)?;
        }
        fmt.write_str(" }")
    }
}

impl mlua::UserData for ArrayBuffer {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(
            mlua::MetaMethod::ToString,
            |_, this: ArrayBuffer| -> Result<String, _> { Ok(format!("{this}")) },
        );
        methods.add_meta_function(
            mlua::MetaMethod::Len,
            |_, this: ArrayBuffer| -> Result<usize, _> { Ok(this.len()) },
        );
    }
}
