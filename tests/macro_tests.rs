use declarative_env::declarative_env;

#[declarative_env(path = "./tests/example.hjson")]
struct ConfigOne;

#[declarative_env(path = "./tests/example.hjson", format = "hjson")]
struct ConfigTwo;

/*
* #[derive(Debug, Clone)]
* pub struct ConfigWithPath {
*    LOG_LEVEL: String,
*    SERVER_PORT: u16,
*    EMPTY_VALUE: String,
*    REQUIRED_VALUE: i32,
* }
*
* impl ConfigWithPath {
*    pub fn from_env() -> Result<Self, Box<dyn Error>> {
*        // TODO: read env variable
*        // if present, use the value,
*        // if not present, use default value (try to parse it before inserting it plain)
*    }
*
*    pub fn LOG_LEVEL(&self) -> &str {
*        &self.LOG_LEVEL
*    }
*
*    pub fn SERVER_PORT(&self) -> u16 {
*        self.SERVER_PORT
*    }
*
*    pub fn EMPTY_VALUE(&self) -> &str {
*        &self.EMPTY_VALUE
*    }
*
*    pub fn REQUIRED_VALUE(&self) -> i32 {
*        self.REQUIRED_VALUE
*    }
* }
*/

#[test]
fn test_config_load() {
    todo!();
}
