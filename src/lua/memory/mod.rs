mod array_buffer;
mod typed_array;

pub use array_buffer::ArrayBuffer;
pub use typed_array::TypedArray;

enum TypedArrayConstructor {
    Default,
    WithLength {
        length: usize,
    },
    WithBuffer {
        buffer: ArrayBuffer,
    },
    WithOffset {
        buffer: ArrayBuffer,
        offset: usize,
    },
    New {
        buffer: ArrayBuffer,
        offset: usize,
        length: usize,
    },
}

impl<'lua> mlua::FromLuaMulti<'lua> for TypedArrayConstructor {
    fn from_lua_multi(values: mlua::MultiValue<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        match values.len() {
            0 => Ok(TypedArrayConstructor::Default),
            1 => {
                let first = &values[0];
                <usize as mlua::FromLua>::from_lua(first.clone(), &lua)
                    .map(|length| TypedArrayConstructor::WithLength { length })
                    .or_else(|_| {
                        <ArrayBuffer as mlua::FromLua>::from_lua(first.clone(), &lua)
                            .map(|buffer| TypedArrayConstructor::WithBuffer { buffer })
                    })
            }
            2 => <(ArrayBuffer, usize) as mlua::FromLuaMulti>::from_lua_multi(values, &lua)
                .map(|(buffer, offset)| TypedArrayConstructor::WithOffset { buffer, offset }),
            3 => <(ArrayBuffer, usize, usize) as mlua::FromLuaMulti>::from_lua_multi(values, &lua)
                .map(|(buffer, offset, length)| TypedArrayConstructor::New {
                    buffer,
                    offset,
                    length,
                }),
            _ => Err(mlua::Error::RuntimeError(format!(
                "couldn't find suitable overload from {values:?}"
            ))),
        }
    }
}

macro_rules! add_typed_array {
    ($lua:ident, $table:ident, $type:ty) => {
        let kind = <$type as typed_array::TypedArrayElement>::kind();
        $table.raw_set(
            format!("{}Array", kind),
            $lua.create_function(
                move |_, args: TypedArrayConstructor| -> Result<TypedArray, _> {
                    match args {
                        TypedArrayConstructor::Default => {
                            TypedArray::with_buffer(kind, array_buffer::ArrayBuffer::default())
                                .map_err(|err| mlua::Error::RuntimeError(err.message()))
                        }
                        TypedArrayConstructor::WithLength { length } => {
                            TypedArray::with_len(kind, length)
                                .map_err(|err| mlua::Error::MemoryError(format!("{err:?}")))
                        }
                        TypedArrayConstructor::WithBuffer { buffer } => {
                            TypedArray::with_buffer(kind, buffer)
                                .map_err(|err| mlua::Error::RuntimeError(err.message()))
                        }
                        TypedArrayConstructor::WithOffset { buffer, offset } => {
                            TypedArray::with_offset(kind, buffer, offset)
                                .map_err(|err| mlua::Error::RuntimeError(err.message()))
                        }
                        TypedArrayConstructor::New {
                            buffer,
                            offset,
                            length,
                        } => TypedArray::new(kind, buffer, offset, length)
                            .map_err(|err| mlua::Error::RuntimeError(err.message())),
                    }
                },
            )?,
        )?;
    };
}

pub fn create_table<'lua>(lua: &'lua mlua::Lua) -> mlua::Result<mlua::Table<'lua>> {
    let memory_table = lua.create_table()?;

    memory_table.raw_set(
        "ArrayBuffer",
        lua.create_function(|_, len: Option<usize>| -> Result<ArrayBuffer, _> {
            if let Some(len) = len {
                ArrayBuffer::new(len).map_err(|_| {
                    mlua::Error::MemoryError(
                        "Failed to allocate memory for the array buffer".into(),
                    )
                })
            } else {
                Ok(ArrayBuffer::default())
            }
        })?,
    )?;

    add_typed_array!(lua, memory_table, i8);
    add_typed_array!(lua, memory_table, u8);

    add_typed_array!(lua, memory_table, i16);
    add_typed_array!(lua, memory_table, u16);

    add_typed_array!(lua, memory_table, i32);
    add_typed_array!(lua, memory_table, u32);

    add_typed_array!(lua, memory_table, i64);
    add_typed_array!(lua, memory_table, u64);

    add_typed_array!(lua, memory_table, f32);
    add_typed_array!(lua, memory_table, f64);

    Ok(memory_table)
}
