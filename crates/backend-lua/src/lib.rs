use mlua::{Function, Lua};

wit_bindgen::generate!({
    world: "backend",
    path: "../../wit",
});

struct Component;

impl Guest for Component {
    fn load(_source: _rt::String) -> ::core::result::Result<(), _rt::String> {
        todo!();
    }

    fn tick() -> ::core::result::Result<(), _rt::String> {
        let lua = Lua::new();
        let globals = lua.globals();

        lua.load(
            r#"
        function greet(name)
            return "Hello from Lua, " .. name .. "!"
        end
        "#,
        )
        .exec()
        .unwrap();

        let greet: Function = globals.get("greet").unwrap();
        let message: String = greet.call("jude love").unwrap();

        Err(message)
    }
}

export!(Component);
