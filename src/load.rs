//! Load (for config)

use toml::Value as TomlValue;

use crate::stacktrace::Stacktrace;



// "have to be implemented" part
pub trait Load {
    const TOML_NAME: &'static str;

    /// MUST be overrided
    /// MUST BE USED ONLY IN `load_from_self_handle_stacktrace`
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self;
}


// "will be auto implemented" part
pub trait LoadAutoImplFns {
    fn load_from_parent_as_root(toml_value: &TomlValue) -> Self;
    fn load_from_parent_handle_stacktrace(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self;
    fn load_from_self_handle_stacktrace(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self;
}


// "public" part
impl<T: Load> LoadAutoImplFns for T {
    fn load_from_parent_as_root(toml_value: &TomlValue) -> Self {
        let stacktrace = Stacktrace::new(Self::TOML_NAME);
        Self::load_from_self(
            toml_value
                .get(Self::TOML_NAME)
                .unwrap_or_else(|| stacktrace.panic_not_found()),
            &stacktrace
        )
    }

    fn load_from_parent_handle_stacktrace(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        let stacktrace = stacktrace.pushed(Self::TOML_NAME);
        Self::load_from_parent(toml_value, &stacktrace)
    }

    fn load_from_self_handle_stacktrace(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        let stacktrace = stacktrace.pushed(Self::TOML_NAME);
        Self::load_from_self(toml_value, &stacktrace)
    }
}


// "private" part
trait LoadAutoImplFnsPrivate {
    fn load_from_parent(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self;
}
impl<T: Load + Sized> LoadAutoImplFnsPrivate for T {
    /// MUST BE USED ONLY IN [`LoadAutoImplFns::load_from_parent_handle_stacktrace`]
    fn load_from_parent(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        assert_eq!(Self::TOML_NAME, stacktrace.last());
        Self::load_from_self(
            toml_value
                .get(Self::TOML_NAME)
                // `stacktrace` isn't updated here, bc it was updated before, in `*_handle_stacktrace`
                .unwrap_or_else(|| stacktrace.panic_not_found()),
            stacktrace
        )
    }
}

