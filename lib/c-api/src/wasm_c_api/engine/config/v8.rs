use crate::{
    error::update_last_error,
    wasm_c_api::engine::{wasm_config_t, wasm_engine_t, wasmer_backend_t},
};

use super::wasmer_backend_config_kind_t;

/// Configuration specific for the `v8` engine.
///
/// This is a Wasmer-specific type with Wasmer-specific functions for
/// manipulating it.
///
/// cbindgen:ignore
#[repr(C)]
#[derive(Debug, Default)]
pub(crate) struct wasmer_v8_engine_config_t;

/// Create a new  [`wasm_engine_t`] backed by a `v8` engine.
pub(crate) fn wasm_v8_engine_new_with_config(config: wasm_config_t) -> Option<Box<wasm_engine_t>> {
    if !matches!(config.backend, wasmer_backend_t::V8) || !config.backend_config.inner.is_v8() {
        update_last_error("Cannot create a new `v8` engine with a non-v8-specific config!");
        return None;
    }

    Some(Box::new(wasm_engine_t {
        inner: wasmer_api::v8::V8::default().into(),
    }))
}

impl wasmer_backend_config_kind_t {
    /// Returns `true` if the wasmer_engine_config_t is [`V8`].
    ///
    /// [`V8`]: wasmer_engine_config_t::V8
    #[must_use]
    pub(super) fn is_v8(&self) -> bool {
        matches!(self, Self::V8(..))
    }
}
