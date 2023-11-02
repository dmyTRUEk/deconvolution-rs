//! Load (for config)

use toml::Value as TomlValue;

use crate::stacktrace::Stacktrace;


pub trait Load {
    const TOML_NAME: &'static str;

    /// must NOT be overrided
    fn load_from_parent_as_root(toml_value: &TomlValue) -> Self
    where Self: Sized
    {
        let stacktrace = Stacktrace::new(Self::TOML_NAME);
        Self::load_from_self(
            toml_value
                .get(Self::TOML_NAME)
                .unwrap_or_else(|| stacktrace.panic_not_found()),
            &stacktrace
        )
    }

    /// must NOT be overrided
    fn load_from_parent_handle_stacktrace(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self
    where Self: Sized
    {
        let stacktrace = stacktrace.pushed(Self::TOML_NAME);
        Self::load_from_parent(toml_value, &stacktrace)
    }

    /// must NOT be overrided
    /// MUST BE USED ONLY IN `load_from_parent_handle_stacktrace`
    fn load_from_parent(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self
    where Self: Sized
    {
        assert_eq!(Self::TOML_NAME, stacktrace.last());
        Self::load_from_self(
            toml_value
                .get(Self::TOML_NAME)
                // `stacktrace` isn't updated, bc it was updated before, in `*_handle_stacktrace`
                .unwrap_or_else(|| stacktrace.panic_not_found()),
            stacktrace
        )
    }

    /// must NOT be overrided
    fn load_from_self_handle_stacktrace(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self
    where Self: Sized
    {
        let stacktrace = stacktrace.pushed(Self::TOML_NAME);
        Self::load_from_self(toml_value, &stacktrace)
    }

    /// MUST be overrided
    /// MUST BE USED ONLY IN `load_from_self_handle_stacktrace`
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self;
}

