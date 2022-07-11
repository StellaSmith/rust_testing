pub trait TypedArrayElement: Sized {
    fn kind() -> TypedArrayKind;

    fn index(buffer: &[u8], byte_offset: usize, index: usize) -> Option<std::ops::Range<usize>> {
        let range = index * core::mem::size_of::<Self>() + byte_offset
            ..(index + 1) * core::mem::size_of::<Self>() + byte_offset;
        buffer.get(range.clone())?;
        Some(range)
    }

    fn get(buffer: &[u8], byte_offset: usize, i: usize) -> Option<Self> {
        let range = Self::index(buffer, byte_offset, i)?;
        let mut result = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            core::intrinsics::copy_nonoverlapping(
                buffer[range].as_ptr() as *const core::mem::MaybeUninit<Self>,
                &mut result as *mut _,
                1,
            )
        };
        Some(unsafe { result.assume_init() })
    }

    fn set(buffer: &[u8], byte_offset: usize, i: usize, this: Self) -> Result<(), ()> {
        let range = Self::index(buffer, byte_offset, i).ok_or(())?;
        unsafe {
            core::intrinsics::copy_nonoverlapping(
                &this as *const Self,
                buffer[range].as_ptr() as *mut Self,
                1,
            )
        };
        Ok(())
    }
}

impl TypedArrayElement for i8 {
    fn kind() -> TypedArrayKind {
        TypedArrayKind::SInt8
    }
}
impl TypedArrayElement for u8 {
    fn kind() -> TypedArrayKind {
        TypedArrayKind::UInt8
    }
}
impl TypedArrayElement for i16 {
    fn kind() -> TypedArrayKind {
        TypedArrayKind::SInt16
    }
}
impl TypedArrayElement for u16 {
    fn kind() -> TypedArrayKind {
        TypedArrayKind::UInt16
    }
}
impl TypedArrayElement for i32 {
    fn kind() -> TypedArrayKind {
        TypedArrayKind::SInt32
    }
}
impl TypedArrayElement for u32 {
    fn kind() -> TypedArrayKind {
        TypedArrayKind::UInt32
    }
}
impl TypedArrayElement for i64 {
    fn kind() -> TypedArrayKind {
        TypedArrayKind::SInt64
    }
}
impl TypedArrayElement for u64 {
    fn kind() -> TypedArrayKind {
        TypedArrayKind::UInt64
    }
}
impl TypedArrayElement for f32 {
    fn kind() -> TypedArrayKind {
        TypedArrayKind::Float32
    }
}
impl TypedArrayElement for f64 {
    fn kind() -> TypedArrayKind {
        TypedArrayKind::Float64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypedArrayKind {
    SInt8,
    UInt8,
    SInt16,
    UInt16,
    SInt32,
    UInt32,
    SInt64,
    UInt64,
    Float32,
    Float64,
}

impl TypedArrayKind {
    pub const fn bytes_per_element(self) -> usize {
        match self {
            TypedArrayKind::SInt8 => core::mem::size_of::<i8>(),
            TypedArrayKind::UInt8 => core::mem::size_of::<u8>(),
            TypedArrayKind::SInt16 => core::mem::size_of::<i16>(),
            TypedArrayKind::UInt16 => core::mem::size_of::<u16>(),
            TypedArrayKind::SInt32 => core::mem::size_of::<i32>(),
            TypedArrayKind::UInt32 => core::mem::size_of::<u32>(),
            TypedArrayKind::SInt64 => core::mem::size_of::<i64>(),
            TypedArrayKind::UInt64 => core::mem::size_of::<u64>(),
            TypedArrayKind::Float32 => core::mem::size_of::<f32>(),
            TypedArrayKind::Float64 => core::mem::size_of::<f64>(),
        }
    }
}

impl std::fmt::Display for TypedArrayKind {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(match self {
            TypedArrayKind::SInt8 => "Int8",
            TypedArrayKind::UInt8 => "UInt8",
            TypedArrayKind::SInt16 => "Int16",
            TypedArrayKind::UInt16 => "UInt16",
            TypedArrayKind::SInt32 => "Int32",
            TypedArrayKind::UInt32 => "UInt32",
            TypedArrayKind::SInt64 => "Int64",
            TypedArrayKind::UInt64 => "UInt64",
            TypedArrayKind::Float32 => "Float32",
            TypedArrayKind::Float64 => "Float64",
        })
    }
}

#[derive(Debug, Clone)]
pub struct TypedArray {
    _kind: TypedArrayKind,
    _buffer: super::ArrayBuffer,
    _offset: usize,
    _length: usize,
}

#[derive(Debug)]
pub enum TypedArrayVariant {
    SInt8(i8),
    UInt8(u8),
    SInt16(i16),
    UInt16(u16),
    SInt32(i32),
    UInt32(u32),
    SInt64(i64),
    UInt64(u64),
    Float32(f32),
    Float64(f64),
}

impl std::fmt::Display for TypedArrayVariant {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypedArrayVariant::SInt8(v) => write!(fmt, "{}", v),
            TypedArrayVariant::UInt8(v) => write!(fmt, "{}", v),
            TypedArrayVariant::SInt16(v) => write!(fmt, "{}", v),
            TypedArrayVariant::UInt16(v) => write!(fmt, "{}", v),
            TypedArrayVariant::SInt32(v) => write!(fmt, "{}", v),
            TypedArrayVariant::UInt32(v) => write!(fmt, "{}", v),
            TypedArrayVariant::SInt64(v) => write!(fmt, "{}", v),
            TypedArrayVariant::UInt64(v) => write!(fmt, "{}", v),
            TypedArrayVariant::Float32(v) => write!(fmt, "{}", v),
            TypedArrayVariant::Float64(v) => write!(fmt, "{}", v),
        }
    }
}

pub struct RangeError(String);

impl RangeError {
    pub(crate) fn new(msg: String) -> Self {
        RangeError(msg)
    }

    pub fn message(self) -> String {
        self.0
    }
}

impl TypedArray {
    pub fn new(
        kind: TypedArrayKind,
        buffer: super::ArrayBuffer,
        offset: usize,
        length: usize,
    ) -> Result<Self, RangeError> {
        if offset % kind.bytes_per_element() != 0 {
            Err(RangeError::new(format!(
                "start offset of {}Array should be a multiple of {}",
                kind,
                kind.bytes_per_element()
            )))
        } else if offset + length * kind.bytes_per_element() > buffer.len() {
            Err(RangeError::new(format!(
                "attempting to construct out-of-bounds {}Array on ArrayBuffer",
                kind
            )))
        } else {
            Ok(TypedArray {
                _kind: kind,
                _buffer: buffer,
                _offset: offset,
                _length: length,
            })
        }
    }

    pub fn with_offset(
        kind: TypedArrayKind,
        buffer: super::ArrayBuffer,
        offset: usize,
    ) -> Result<Self, RangeError> {
        buffer
            .len()
            .checked_sub(offset)
            .filter(|byte_length| byte_length % kind.bytes_per_element() == 0)
            .map(|byte_length| byte_length / kind.bytes_per_element())
            .map(|length| TypedArray {
                _kind: kind,
                _buffer: buffer,
                _offset: offset,
                _length: length,
            })
            .ok_or_else(|| {
                RangeError::new(format!(
                    "attempting to construct out-of-bounds {}Array on ArrayBuffer",
                    kind
                ))
            })
    }

    pub fn with_len(
        kind: TypedArrayKind,
        length: usize,
    ) -> Result<Self, std::collections::TryReserveError> {
        let buffer = super::ArrayBuffer::new(length * kind.bytes_per_element())?;
        Ok(TypedArray {
            _kind: kind,
            _buffer: buffer,
            _offset: 0,
            _length: length,
        })
    }

    pub fn with_buffer(
        kind: TypedArrayKind,
        buffer: super::ArrayBuffer,
    ) -> Result<Self, RangeError> {
        if buffer.len() % kind.bytes_per_element() != 0 {
            Err(RangeError::new(format!(
                "buffer length for {}Array should be a multiple of {}",
                kind,
                kind.bytes_per_element()
            )))
        } else {
            Ok(TypedArray {
                _kind: kind,
                _length: buffer.len() / kind.bytes_per_element(),
                _buffer: buffer,
                _offset: 0,
            })
        }
    }

    pub fn byte_offset(&self) -> usize {
        self._offset
    }

    pub fn byte_len(&self) -> usize {
        self._length * self._kind.bytes_per_element()
    }

    pub fn len(&self) -> usize {
        self._length
    }

    pub fn buffer(&self) -> super::ArrayBuffer {
        self._buffer.clone()
    }

    pub fn name(&self) -> &'static str {
        match self._kind {
            TypedArrayKind::SInt8 => "Int8Array",
            TypedArrayKind::UInt8 => "UInt8Array",
            TypedArrayKind::SInt16 => "Int16Array",
            TypedArrayKind::UInt16 => "UInt16Array",
            TypedArrayKind::SInt32 => "Int32Array",
            TypedArrayKind::UInt32 => "UInt32Array",
            TypedArrayKind::SInt64 => "Int64Array",
            TypedArrayKind::UInt64 => "UInt64Array",
            TypedArrayKind::Float32 => "Float32Array",
            TypedArrayKind::Float64 => "Float64Array",
        }
    }

    pub fn get_number(&self, index: usize) -> Option<mlua::Number> {
        let result = Some(match self._kind {
            TypedArrayKind::SInt8 => {
                <u8 as TypedArrayElement>::get(self._buffer.slice(), self._offset, index)?
                    as mlua::Number
            }
            TypedArrayKind::UInt8 => {
                <i8 as TypedArrayElement>::get(self._buffer.slice(), self._offset, index)?
                    as mlua::Number
            }
            TypedArrayKind::SInt16 => {
                <u16 as TypedArrayElement>::get(self._buffer.slice(), self._offset, index)?
                    as mlua::Number
            }
            TypedArrayKind::UInt16 => {
                <i16 as TypedArrayElement>::get(self._buffer.slice(), self._offset, index)?
                    as mlua::Number
            }
            TypedArrayKind::SInt32 => {
                <u32 as TypedArrayElement>::get(self._buffer.slice(), self._offset, index)?
                    as mlua::Number
            }
            TypedArrayKind::UInt32 => {
                <i32 as TypedArrayElement>::get(self._buffer.slice(), self._offset, index)?
                    as mlua::Number
            }
            TypedArrayKind::SInt64 => {
                <u64 as TypedArrayElement>::get(self._buffer.slice(), self._offset, index)?
                    as mlua::Number
            }
            TypedArrayKind::UInt64 => {
                <i64 as TypedArrayElement>::get(self._buffer.slice(), self._offset, index)?
                    as mlua::Number
            }
            TypedArrayKind::Float32 => {
                <f32 as TypedArrayElement>::get(self._buffer.slice(), self._offset, index)?
                    as mlua::Number
            }
            TypedArrayKind::Float64 => {
                <f64 as TypedArrayElement>::get(self._buffer.slice(), self._offset, index)?
                    as mlua::Number
            }
        });
        result
    }

    pub unsafe fn unsafe_set<T: TypedArrayElement>(&mut self, index: usize, number: T) {
        T::set(self._buffer.slice(), self._offset, index, number).unwrap_unchecked()
    }

    pub fn set<T: TypedArrayElement>(&mut self, index: usize, number: T) -> Result<(), ()> {
        if self._kind != T::kind() {
            Err(())
        } else {
            T::set(self._buffer.slice(), self._offset, index, number)
        }
    }

    pub fn set_number(&mut self, index: usize, number: mlua::Number) -> Result<(), ()> {
        match self._kind {
            TypedArrayKind::SInt8 => <i8 as TypedArrayElement>::set(
                self._buffer.slice(),
                self._offset,
                index,
                number as _,
            ),
            TypedArrayKind::UInt8 => <u8 as TypedArrayElement>::set(
                self._buffer.slice(),
                self._offset,
                index,
                number as _,
            ),
            TypedArrayKind::SInt16 => <i16 as TypedArrayElement>::set(
                self._buffer.slice(),
                self._offset,
                index,
                number as _,
            ),
            TypedArrayKind::UInt16 => <u16 as TypedArrayElement>::set(
                self._buffer.slice(),
                self._offset,
                index,
                number as _,
            ),
            TypedArrayKind::SInt32 => <i32 as TypedArrayElement>::set(
                self._buffer.slice(),
                self._offset,
                index,
                number as _,
            ),
            TypedArrayKind::UInt32 => <u32 as TypedArrayElement>::set(
                self._buffer.slice(),
                self._offset,
                index,
                number as _,
            ),
            TypedArrayKind::SInt64 => <i64 as TypedArrayElement>::set(
                self._buffer.slice(),
                self._offset,
                index,
                number as _,
            ),
            TypedArrayKind::UInt64 => <u64 as TypedArrayElement>::set(
                self._buffer.slice(),
                self._offset,
                index,
                number as _,
            ),
            TypedArrayKind::Float32 => <f32 as TypedArrayElement>::set(
                self._buffer.slice(),
                self._offset,
                index,
                number as _,
            ),
            TypedArrayKind::Float64 => <f64 as TypedArrayElement>::set(
                self._buffer.slice(),
                self._offset,
                index,
                number as _,
            ),
        }
    }

    pub unsafe fn unsafe_get<T: TypedArrayElement>(&self, index: usize) -> T {
        T::get(self._buffer.slice(), self._offset, index).unwrap_unchecked()
    }

    pub fn get<T: TypedArrayElement>(&self, index: usize) -> Option<T> {
        if T::kind() != self._kind {
            return None;
        };
        T::get(self._buffer.slice(), self._offset, index)
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = TypedArrayVariant> + 'a {
        (0..self.len()).map(|i| match self._kind {
            TypedArrayKind::SInt8 => TypedArrayVariant::SInt8(unsafe { self.unsafe_get::<i8>(i) }),
            TypedArrayKind::UInt8 => TypedArrayVariant::UInt8(unsafe { self.unsafe_get::<u8>(i) }),
            TypedArrayKind::SInt16 => {
                TypedArrayVariant::SInt16(unsafe { self.unsafe_get::<i16>(i) })
            }
            TypedArrayKind::UInt16 => {
                TypedArrayVariant::UInt16(unsafe { self.unsafe_get::<u16>(i) })
            }
            TypedArrayKind::SInt32 => {
                TypedArrayVariant::SInt32(unsafe { self.get::<i32>(i).unwrap_unchecked() })
            }
            TypedArrayKind::UInt32 => {
                TypedArrayVariant::UInt32(unsafe { self.unsafe_get::<u32>(i) })
            }
            TypedArrayKind::SInt64 => {
                TypedArrayVariant::SInt64(unsafe { self.unsafe_get::<i64>(i) })
            }
            TypedArrayKind::UInt64 => {
                TypedArrayVariant::UInt64(unsafe { self.unsafe_get::<u64>(i) })
            }
            TypedArrayKind::Float32 => {
                TypedArrayVariant::Float32(unsafe { self.unsafe_get::<f32>(i) })
            }
            TypedArrayKind::Float64 => {
                TypedArrayVariant::Float64(unsafe { self.unsafe_get::<f64>(i) })
            }
        })
    }
}

impl std::fmt::Display for TypedArray {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(self.name())?;
        fmt.write_str(" { ")?;
        for x in self.iter() {
            write!(fmt, "{}, ", x)?;
        }
        fmt.write_str(" }")
    }
}

impl mlua::UserData for TypedArray {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.name()));
        fields.add_field_method_get("BYTES_PER_ELEMENT", |_, this| {
            Ok(this._kind.bytes_per_element())
        });
        fields.add_field_method_get("buffer", |_, this| Ok(this._buffer.clone()));
        fields.add_field_method_get("byteLength", |_, this| Ok(this.byte_len()));
        fields.add_field_method_get("byteOffset", |_, this| Ok(this.byte_offset()));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(
            mlua::MetaMethod::ToString,
            |_, this: TypedArray| -> Result<String, _> { Ok(format!("{this}")) },
        );
        methods.add_meta_function(
            mlua::MetaMethod::Len,
            |_, this: TypedArray| -> Result<usize, _> { Ok(this.len()) },
        );
        methods.add_meta_function(
            mlua::MetaMethod::Index,
            |_, args: (TypedArray, usize)| -> Result<Option<mlua::Number>, _> {
                let (this, index) = args;
                Ok(index
                    .checked_sub(1)
                    .and_then(|index| this.get_number(index)))
            },
        );
        methods.add_meta_function(
            mlua::MetaMethod::NewIndex,
            |_, args: (TypedArray, usize, mlua::Number)| -> Result<(), _> {
                let (mut this, index, number) = args;
                index
                    .checked_sub(1)
                    .and_then(move |index| this.set_number(index, number).ok())
                    .ok_or_else(|| {
                        mlua::Error::RuntimeError("assignment index out of range".into())
                    })
            },
        );
        // methods.add_meta_function(
        //     mlua::MetaMethod::Iter,
        //     |_, args: (TypedArray, usize, mlua::Number)| -> Result<(), _> {
        //         let (mut this, index, number) = args;
        //         index
        //             .checked_sub(1)
        //             .and_then(move |index| this.set_number(index, number).ok())
        //             .ok_or_else(|| {
        //                 mlua::Error::RuntimeError("assignment index out of range".into())
        //             })
        //     },
        // );
    }
}
