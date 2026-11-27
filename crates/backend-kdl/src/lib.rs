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
        Ok(())
    }
}

export!(Component);
