extern crate mlua;

pub mod lua;

fn exec<'lua>(lua: &'lua mlua::Lua) {
    let result = || -> mlua::Result<()> {
        let chunk = mlua::chunk! {
            local function toUTF32(s)
                local len = utf8.len(s)
                local buff = memory.UInt32Array(len)
                local i = 1
                for _, c in utf8.codes(s) do
                    buff[i] = c
                    i = i + 1
                end
                return buff
            end
            local function toUTF8(b)
                local s = ""
                for i = 1, #b do
                    s = s .. utf8.char(b[i])
                end
                return s
            end
            local s = "Y (ᗜˬᗜ) g ii"
            local b = toUTF32(s)

            for k, v in ipairs(b) do print(k,v) end
        };
        lua.load(chunk).exec()?;
        Ok(())
    }();
    if let Err(ref err) = &result {
        eprintln!("{}", err);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lua = unsafe { mlua::Lua::unsafe_new() };

    lua.globals()
        .set("memory", lua::memory::create_table(&lua)?)?;

    exec(&lua);

    Ok(())
}
