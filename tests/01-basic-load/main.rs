use declarative_env::declarative_env;

#[declarative_env(path = "./tests/01-basic-load/spec.hjson", format = "hjson")]
struct MyConfig;

