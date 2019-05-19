pub struct User {
    pub name: String,
    pub password: String,
    pub age: u32,
}

impl User {
    pub fn new(name: String, password: String, age: u32) -> User {
        User {
            name,
            password,
            age,
        }
    }

    pub fn is_authorized(&self, password: &str) -> bool {
        self.password == password
    }
}
