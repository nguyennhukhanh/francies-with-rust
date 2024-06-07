pub struct AuthService;

impl AuthService {
    pub fn new() -> Self {
        AuthService
    }

    pub fn authenticate(&self, email: &str, password: &str) -> bool {
        email == "admin@gmail.com" && password == "123456789"
    }
}