# declarative-env
This crate contains a macro for generating a struct which reads the environmental variables defined in the configured file.

This allows to have a single point of definition for the env configuration, thus avoiding a possible incoherence between the documentation and the implementation.

# Usage
The macro must be attached to an empty struct definition:
```no_run
   #[declarative_env!(path = "./example.hjson", format = "hjson")]
   struct MyConfig;
```
> Note: the struct can have any visibility.

The macro creates a struct with the same name of the empty struct, but it injects as fields all the variables defined in the `path` keyword.

For example, the definition file:
```hjson
SERVER_PORT: {
   type: u16
   description: The port the server will listen on
   default: 8080
}
```
generates the struct
```
struct MyConfig {
   SERVER_PORT: u16,
}
```

The path of the definition file is relative to the cargo manifest dir (the one with
`Cargo.toml`).

The supported formats are specified in the `AcceptedFormat` enum; formats are the lowercase version of the enum variants, so `AcceptedFormat::Hjson` is selected with `format = "hjson"`.

## Env Variable Definition
The env variable definition file contains a map of objects, where the key is the name of the
env variable, and the value contains the metadata of the variable:
- the **type** (accepted types are defined in `AcceptedRustType`).
- the **description**, unused in generated code, required for documentation purpose.
- the **default** value, optional (it must be parsable as the defiend **type**).

### Accepted formats
Supported formats are listed in the [AcceptedFormat enum](./src/macro_config.rs).

### Accepted types
Types variables can be are listed in the [AcceptedRustType enum](./src/accepted_rust_type.rs).

## License
This project is distributed according to the `Mozilla Public License 2.0`.
