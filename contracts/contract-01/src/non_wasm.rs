use near_sdk::{okapi::openapi3::Server, openapi};

pub mod mods {
    pub use crate::{self as counter, init, version};
}

pub mod methods {
    use super::mods;
    pub use crate::_methods::*;
    pub use mods::init::_methods::*;
    pub use mods::version::IVersion_methods::version as _version;
}

pub fn openapi() -> near_sdk::okapi::openapi3::OpenApi {
    let mut spec = routes();
    spec.info = info();
    spec.servers = servers();
    spec
}

pub fn servers() -> Vec<Server> {
    vec![
        Server {
            url: "https://explorer.testnet.near.org/accounts/todo.testnet".into(),
            description: Some("TODO testnet account".into()),
            variables: [].into(),
            extensions: [].into(),
        },
        Server {
            url: "https://explorer.near.org/accounts/todo.near".into(),
            description: Some("TODO mainnet account".into()),
            variables: [].into(),
            extensions: [].into(),
        },
    ]
}

pub fn info() -> near_sdk::okapi::openapi3::Info {
    use common::version::Version;
    let version = common::version::version_from_env!();

    near_sdk::okapi::openapi3::Info {
        title: version.name,
        version: version.semver,
        description: Some(format!(
            "    - Git SHA: {}\n    - Git Datetime: {}\n    - Git Dirty: {}\n    - Cargo Features: {}",
            version.git_sha, version.git_datetime, version.git_dirty, version.cargo_features,
        )),
        ..Default::default()
    }
}

pub fn routes() -> near_sdk::okapi::openapi3::OpenApi {
    use openapi::OperationAdd;

    let settings = near_sdk::schemars::gen::SchemaSettings::openapi3_1();
    let mut gen = openapi::OpenApiGenerator::new(&settings);

    let tag_names = ["init", "counter", "misc"];

    // endpoints/methods
    {
        let gen = &mut gen;
        let tags: std::collections::HashMap<_, _> =
            tag_names.into_iter().map(|tag| (tag, tag)).collect();

        // init
        let tag = &[tags["init"]];
        use mods::init;
        init::new::Input::operation_add(gen, tag);
        init::new_with::Input::operation_add(gen, tag);
        // counter
        let tag = &[tags["counter"]];
        use mods::counter;
        counter::increment::Input::operation_add(gen, tag);
        counter::decrement::Input::operation_add(gen, tag);
        counter::get::Input::operation_add(gen, tag);
        counter::get_as_integer::Input::operation_add(gen, tag);
        counter::set::Input::operation_add(gen, tag);
        counter::log_value::Input::operation_add(gen, tag);
        // misc
        let tag = &[tags["misc"]];
        use mods::version;
        version::version::Input::operation_add(gen, tag);
    }

    gen.into_openapi_with_tags(&tag_names)
}
