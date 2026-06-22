pub struct Database {
    path: String,
}

impl Database {
    pub fn open(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { path: path.to_string() })
    }
    
    pub fn execute(&self, _sql: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    
    pub fn query<T>(&self, _sql: &str) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }
}
