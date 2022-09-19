use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("PROTOC", "./proto/bin/protoc");
    // tonic_build::compile_protos("proto/command.proto").unwrap();
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize, my_macro::MyName)]")
        .field_attribute("data", "#[serde(skip_serializing_if = \"Vec::is_empty\")]")
        .field_attribute(".", "#[serde(default)]")
        .compile(&["proto/command.proto"], &["proto"])
        .unwrap();
    Ok(())
}