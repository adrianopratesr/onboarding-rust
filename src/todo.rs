pub struct Todo {
    pub message: String,
}

impl Todo {
    pub const fn new(message: String) -> Self {
        Self { message }
    }
}
