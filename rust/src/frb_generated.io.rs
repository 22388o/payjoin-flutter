// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.24.

// Section: imports

use super::*;
use crate::api::receive::*;
use crate::api::send::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate_io!();

// Section: dart2rust

impl CstDecode<flutter_rust_bridge::DartOpaque> for *const std::ffi::c_void {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> flutter_rust_bridge::DartOpaque {
        unsafe { flutter_rust_bridge::for_generated::cst_decode_dart_opaque(self as _) }
    }
}
impl CstDecode<std::collections::HashMap<String, String>>
    for *mut wire_cst_list_record_string_string
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> std::collections::HashMap<String, String> {
        let vec: Vec<(String, String)> = self.cst_decode();
        vec.into_iter().collect()
    }
}
impl CstDecode<RustOpaqueNom<Arc<payjoin_ffi::receive::v1::MaybeInputsOwned>>> for usize {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> RustOpaqueNom<Arc<payjoin_ffi::receive::v1::MaybeInputsOwned>> {
        unsafe { decode_rust_opaque_nom(self as _) }
    }
}
impl CstDecode<RustOpaqueNom<Arc<payjoin_ffi::send::v1::ContextV1>>> for usize {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> RustOpaqueNom<Arc<payjoin_ffi::send::v1::ContextV1>> {
        unsafe { decode_rust_opaque_nom(self as _) }
    }
}
impl CstDecode<RustOpaqueNom<Arc<payjoin_ffi::send::v1::RequestBuilder>>> for usize {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> RustOpaqueNom<Arc<payjoin_ffi::send::v1::RequestBuilder>> {
        unsafe { decode_rust_opaque_nom(self as _) }
    }
}
impl CstDecode<RustOpaqueNom<Arc<payjoin_ffi::send::v1::RequestContext>>> for usize {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> RustOpaqueNom<Arc<payjoin_ffi::send::v1::RequestContext>> {
        unsafe { decode_rust_opaque_nom(self as _) }
    }
}
impl CstDecode<RustOpaqueNom<Arc<payjoin_ffi::send::v2::ContextV2>>> for usize {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> RustOpaqueNom<Arc<payjoin_ffi::send::v2::ContextV2>> {
        unsafe { decode_rust_opaque_nom(self as _) }
    }
}
impl CstDecode<RustOpaqueNom<Arc<payjoin_ffi::uri::Uri>>> for usize {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> RustOpaqueNom<Arc<payjoin_ffi::uri::Uri>> {
        unsafe { decode_rust_opaque_nom(self as _) }
    }
}
impl CstDecode<RustOpaqueNom<Arc<payjoin_ffi::uri::Url>>> for usize {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> RustOpaqueNom<Arc<payjoin_ffi::uri::Url>> {
        unsafe { decode_rust_opaque_nom(self as _) }
    }
}
impl CstDecode<RustOpaqueNom<payjoin_ffi::receive::v1::UncheckedProposal>> for usize {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> RustOpaqueNom<payjoin_ffi::receive::v1::UncheckedProposal> {
        unsafe { decode_rust_opaque_nom(self as _) }
    }
}
impl CstDecode<String> for *mut wire_cst_list_prim_u_8_strict {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> String {
        let vec: Vec<u8> = self.cst_decode();
        String::from_utf8(vec).unwrap()
    }
}
impl CstDecode<crate::api::send::ContextV1> for *mut wire_cst_context_v_1 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::send::ContextV1 {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::api::send::ContextV1>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::api::send::ContextV2> for *mut wire_cst_context_v_2 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::send::ContextV2 {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::api::send::ContextV2>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::api::receive::Headers> for *mut wire_cst_headers {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::receive::Headers {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::api::receive::Headers>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::api::send::RequestBuilder> for *mut wire_cst_request_builder {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::send::RequestBuilder {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::api::send::RequestBuilder>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::api::send::RequestContext> for *mut wire_cst_request_context {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::send::RequestContext {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::api::send::RequestContext>::cst_decode(*wrap).into()
    }
}
impl CstDecode<u64> for *mut u64 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> u64 {
        unsafe { *flutter_rust_bridge::for_generated::box_from_leak_ptr(self) }
    }
}
impl CstDecode<u8> for *mut u8 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> u8 {
        unsafe { *flutter_rust_bridge::for_generated::box_from_leak_ptr(self) }
    }
}
impl CstDecode<crate::api::receive::UncheckedProposal> for *mut wire_cst_unchecked_proposal {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::receive::UncheckedProposal {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::api::receive::UncheckedProposal>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::api::uri::Uri> for *mut wire_cst_uri {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::uri::Uri {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::api::uri::Uri>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::api::uri::Url> for *mut wire_cst_url {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::uri::Url {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::api::uri::Url>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::api::send::ContextV1> for wire_cst_context_v_1 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::send::ContextV1 {
        crate::api::send::ContextV1(self.field0.cst_decode())
    }
}
impl CstDecode<crate::api::send::ContextV2> for wire_cst_context_v_2 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::send::ContextV2 {
        crate::api::send::ContextV2(self.field0.cst_decode())
    }
}
impl CstDecode<crate::api::receive::Headers> for wire_cst_headers {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::receive::Headers {
        crate::api::receive::Headers(self.field0.cst_decode())
    }
}
impl CstDecode<Vec<u8>> for *mut wire_cst_list_prim_u_8_loose {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<u8> {
        unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl CstDecode<Vec<u8>> for *mut wire_cst_list_prim_u_8_strict {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<u8> {
        unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl CstDecode<Vec<(String, String)>> for *mut wire_cst_list_record_string_string {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<(String, String)> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(CstDecode::cst_decode).collect()
    }
}
impl CstDecode<crate::api::receive::MaybeInputsOwned> for wire_cst_maybe_inputs_owned {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::receive::MaybeInputsOwned {
        crate::api::receive::MaybeInputsOwned(self.field0.cst_decode())
    }
}
impl CstDecode<crate::utils::error::PayjoinError> for wire_cst_payjoin_error {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::utils::error::PayjoinError {
        match self.tag {
            0 => {
                let ans = unsafe { self.kind.InvalidAddress };
                crate::utils::error::PayjoinError::InvalidAddress {
                    message: ans.message.cst_decode(),
                }
            }
            1 => {
                let ans = unsafe { self.kind.InvalidScript };
                crate::utils::error::PayjoinError::InvalidScript {
                    message: ans.message.cst_decode(),
                }
            }
            2 => {
                let ans = unsafe { self.kind.NetworkValidation };
                crate::utils::error::PayjoinError::NetworkValidation {
                    message: ans.message.cst_decode(),
                }
            }
            3 => {
                let ans = unsafe { self.kind.PsbtParseError };
                crate::utils::error::PayjoinError::PsbtParseError {
                    message: ans.message.cst_decode(),
                }
            }
            4 => {
                let ans = unsafe { self.kind.ResponseError };
                crate::utils::error::PayjoinError::ResponseError {
                    message: ans.message.cst_decode(),
                }
            }
            5 => {
                let ans = unsafe { self.kind.RequestError };
                crate::utils::error::PayjoinError::RequestError {
                    message: ans.message.cst_decode(),
                }
            }
            6 => {
                let ans = unsafe { self.kind.TransactionError };
                crate::utils::error::PayjoinError::TransactionError {
                    message: ans.message.cst_decode(),
                }
            }
            7 => {
                let ans = unsafe { self.kind.ServerError };
                crate::utils::error::PayjoinError::ServerError {
                    message: ans.message.cst_decode(),
                }
            }
            8 => {
                let ans = unsafe { self.kind.SelectionError };
                crate::utils::error::PayjoinError::SelectionError {
                    message: ans.message.cst_decode(),
                }
            }
            9 => {
                let ans = unsafe { self.kind.CreateRequestError };
                crate::utils::error::PayjoinError::CreateRequestError {
                    message: ans.message.cst_decode(),
                }
            }
            10 => {
                let ans = unsafe { self.kind.PjParseError };
                crate::utils::error::PayjoinError::PjParseError {
                    message: ans.message.cst_decode(),
                }
            }
            11 => {
                let ans = unsafe { self.kind.PjNotSupported };
                crate::utils::error::PayjoinError::PjNotSupported {
                    message: ans.message.cst_decode(),
                }
            }
            12 => {
                let ans = unsafe { self.kind.ValidationError };
                crate::utils::error::PayjoinError::ValidationError {
                    message: ans.message.cst_decode(),
                }
            }
            13 => {
                let ans = unsafe { self.kind.V2Error };
                crate::utils::error::PayjoinError::V2Error {
                    message: ans.message.cst_decode(),
                }
            }
            14 => {
                let ans = unsafe { self.kind.UnexpectedError };
                crate::utils::error::PayjoinError::UnexpectedError {
                    message: ans.message.cst_decode(),
                }
            }
            15 => {
                let ans = unsafe { self.kind.OhttpError };
                crate::utils::error::PayjoinError::OhttpError {
                    message: ans.message.cst_decode(),
                }
            }
            16 => {
                let ans = unsafe { self.kind.UrlError };
                crate::utils::error::PayjoinError::UrlError {
                    message: ans.message.cst_decode(),
                }
            }
            _ => unreachable!(),
        }
    }
}
impl CstDecode<(String, String)> for wire_cst_record_string_string {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> (String, String) {
        (self.field0.cst_decode(), self.field1.cst_decode())
    }
}
impl CstDecode<crate::api::send::Request> for wire_cst_request {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::send::Request {
        crate::api::send::Request {
            url: self.url.cst_decode(),
            body: self.body.cst_decode(),
        }
    }
}
impl CstDecode<crate::api::send::RequestBuilder> for wire_cst_request_builder {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::send::RequestBuilder {
        crate::api::send::RequestBuilder(self.field0.cst_decode())
    }
}
impl CstDecode<crate::api::send::RequestContext> for wire_cst_request_context {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::send::RequestContext {
        crate::api::send::RequestContext(self.field0.cst_decode())
    }
}
impl CstDecode<crate::api::send::RequestContextV1> for wire_cst_request_context_v_1 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::send::RequestContextV1 {
        crate::api::send::RequestContextV1 {
            request: self.request.cst_decode(),
            context_v1: self.context_v1.cst_decode(),
        }
    }
}
impl CstDecode<crate::api::send::RequestContextV2> for wire_cst_request_context_v_2 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::send::RequestContextV2 {
        crate::api::send::RequestContextV2 {
            request: self.request.cst_decode(),
            context_v2: self.context_v2.cst_decode(),
        }
    }
}
impl CstDecode<crate::api::receive::UncheckedProposal> for wire_cst_unchecked_proposal {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::receive::UncheckedProposal {
        crate::api::receive::UncheckedProposal(self.field0.cst_decode())
    }
}
impl CstDecode<crate::api::uri::Uri> for wire_cst_uri {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::uri::Uri {
        crate::api::uri::Uri(self.field0.cst_decode())
    }
}
impl CstDecode<crate::api::uri::Url> for wire_cst_url {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::uri::Url {
        crate::api::uri::Url(self.field0.cst_decode())
    }
}
impl NewWithNullPtr for wire_cst_context_v_1 {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}
impl Default for wire_cst_context_v_1 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_context_v_2 {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}
impl Default for wire_cst_context_v_2 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_headers {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_cst_headers {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_maybe_inputs_owned {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}
impl Default for wire_cst_maybe_inputs_owned {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_payjoin_error {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: PayjoinErrorKind { nil__: () },
        }
    }
}
impl Default for wire_cst_payjoin_error {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_record_string_string {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
            field1: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_cst_record_string_string {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_request {
    fn new_with_null_ptr() -> Self {
        Self {
            url: Default::default(),
            body: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_cst_request {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_request_builder {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}
impl Default for wire_cst_request_builder {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_request_context {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}
impl Default for wire_cst_request_context {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_request_context_v_1 {
    fn new_with_null_ptr() -> Self {
        Self {
            request: Default::default(),
            context_v1: Default::default(),
        }
    }
}
impl Default for wire_cst_request_context_v_1 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_request_context_v_2 {
    fn new_with_null_ptr() -> Self {
        Self {
            request: Default::default(),
            context_v2: Default::default(),
        }
    }
}
impl Default for wire_cst_request_context_v_2 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_unchecked_proposal {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}
impl Default for wire_cst_unchecked_proposal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_uri {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}
impl Default for wire_cst_uri {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_url {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}
impl Default for wire_cst_url {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_Headers_from_vec(
    port_: i64,
    body: *mut wire_cst_list_prim_u_8_loose,
) {
    wire_Headers_from_vec_impl(port_, body)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_UncheckedProposal_check_broadcast_suitability(
    port_: i64,
    that: *mut wire_cst_unchecked_proposal,
    min_fee_rate: *mut u64,
    can_broadcast: *const std::ffi::c_void,
) {
    wire_UncheckedProposal_check_broadcast_suitability_impl(
        port_,
        that,
        min_fee_rate,
        can_broadcast,
    )
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_UncheckedProposal_extract_tx_to_schedule_broadcast(
    port_: i64,
    that: *mut wire_cst_unchecked_proposal,
) {
    wire_UncheckedProposal_extract_tx_to_schedule_broadcast_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_UncheckedProposal_from_request(
    port_: i64,
    body: *mut wire_cst_list_prim_u_8_loose,
    query: *mut wire_cst_list_prim_u_8_strict,
    headers: *mut wire_cst_headers,
) {
    wire_UncheckedProposal_from_request_impl(port_, body, query, headers)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_ContextV1_process_response(
    port_: i64,
    that: *mut wire_cst_context_v_1,
    response: *mut wire_cst_list_prim_u_8_loose,
) {
    wire_ContextV1_process_response_impl(port_, that, response)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_ContextV2_process_response(
    port_: i64,
    that: *mut wire_cst_context_v_2,
    response: *mut wire_cst_list_prim_u_8_loose,
) {
    wire_ContextV2_process_response_impl(port_, that, response)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_RequestBuilder_always_disable_output_substitution(
    port_: i64,
    that: *mut wire_cst_request_builder,
    disable: bool,
) {
    wire_RequestBuilder_always_disable_output_substitution_impl(port_, that, disable)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_RequestBuilder_build_non_incentivizing(
    port_: i64,
    that: *mut wire_cst_request_builder,
) {
    wire_RequestBuilder_build_non_incentivizing_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_RequestBuilder_build_recommended(
    port_: i64,
    that: *mut wire_cst_request_builder,
    min_fee_rate: u64,
) {
    wire_RequestBuilder_build_recommended_impl(port_, that, min_fee_rate)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_RequestBuilder_build_with_additional_fee(
    port_: i64,
    that: *mut wire_cst_request_builder,
    max_fee_contribution: u64,
    change_index: *mut u8,
    min_fee_rate: u64,
    clamp_fee_contribution: bool,
) {
    wire_RequestBuilder_build_with_additional_fee_impl(
        port_,
        that,
        max_fee_contribution,
        change_index,
        min_fee_rate,
        clamp_fee_contribution,
    )
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_RequestBuilder_from_psbt_and_uri(
    port_: i64,
    psbt_base64: *mut wire_cst_list_prim_u_8_strict,
    uri: *mut wire_cst_uri,
) {
    wire_RequestBuilder_from_psbt_and_uri_impl(port_, psbt_base64, uri)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_RequestBuilder_new(port_: i64) {
    wire_RequestBuilder_new_impl(port_)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_RequestContext_extract_v1(
    port_: i64,
    that: *mut wire_cst_request_context,
) {
    wire_RequestContext_extract_v1_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_RequestContext_extract_v2(
    port_: i64,
    that: *mut wire_cst_request_context,
    ohttp_proxy_url: *mut wire_cst_list_prim_u_8_strict,
) {
    wire_RequestContext_extract_v2_impl(port_, that, ohttp_proxy_url)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_Uri_address(port_: i64, that: *mut wire_cst_uri) {
    wire_Uri_address_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_Uri_amount(port_: i64, that: *mut wire_cst_uri) {
    wire_Uri_amount_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_Uri_from_str(
    port_: i64,
    uri: *mut wire_cst_list_prim_u_8_strict,
) {
    wire_Uri_from_str_impl(port_, uri)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_Url_new(
    port_: i64,
    input: *mut wire_cst_list_prim_u_8_strict,
) {
    wire_Url_new_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_wire_Url_query(port_: i64, that: *mut wire_cst_url) {
    wire_Url_query_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_rust_arc_increment_strong_count_RustOpaque_Arcpayjoin_ffireceivev1MaybeInputsOwned(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<Arc<payjoin_ffi::receive::v1::MaybeInputsOwned>>::increment_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_rust_arc_decrement_strong_count_RustOpaque_Arcpayjoin_ffireceivev1MaybeInputsOwned(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<Arc<payjoin_ffi::receive::v1::MaybeInputsOwned>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_rust_arc_increment_strong_count_RustOpaque_Arcpayjoin_ffisendv1ContextV1(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<Arc<payjoin_ffi::send::v1::ContextV1>>::increment_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_rust_arc_decrement_strong_count_RustOpaque_Arcpayjoin_ffisendv1ContextV1(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<Arc<payjoin_ffi::send::v1::ContextV1>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_rust_arc_increment_strong_count_RustOpaque_Arcpayjoin_ffisendv1RequestBuilder(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<Arc<payjoin_ffi::send::v1::RequestBuilder>>::increment_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_rust_arc_decrement_strong_count_RustOpaque_Arcpayjoin_ffisendv1RequestBuilder(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<Arc<payjoin_ffi::send::v1::RequestBuilder>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_rust_arc_increment_strong_count_RustOpaque_Arcpayjoin_ffisendv1RequestContext(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<Arc<payjoin_ffi::send::v1::RequestContext>>::increment_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_rust_arc_decrement_strong_count_RustOpaque_Arcpayjoin_ffisendv1RequestContext(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<Arc<payjoin_ffi::send::v1::RequestContext>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_rust_arc_increment_strong_count_RustOpaque_Arcpayjoin_ffisendv2ContextV2(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<Arc<payjoin_ffi::send::v2::ContextV2>>::increment_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_rust_arc_decrement_strong_count_RustOpaque_Arcpayjoin_ffisendv2ContextV2(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<Arc<payjoin_ffi::send::v2::ContextV2>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_rust_arc_increment_strong_count_RustOpaque_Arcpayjoin_ffiuriUri(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<Arc<payjoin_ffi::uri::Uri>>::increment_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_rust_arc_decrement_strong_count_RustOpaque_Arcpayjoin_ffiuriUri(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<Arc<payjoin_ffi::uri::Uri>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_rust_arc_increment_strong_count_RustOpaque_Arcpayjoin_ffiuriUrl(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<Arc<payjoin_ffi::uri::Url>>::increment_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_rust_arc_decrement_strong_count_RustOpaque_Arcpayjoin_ffiuriUrl(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<Arc<payjoin_ffi::uri::Url>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_rust_arc_increment_strong_count_RustOpaque_payjoin_ffireceivev1UncheckedProposal(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<payjoin_ffi::receive::v1::UncheckedProposal>::increment_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_rust_arc_decrement_strong_count_RustOpaque_payjoin_ffireceivev1UncheckedProposal(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<payjoin_ffi::receive::v1::UncheckedProposal>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_cst_new_box_autoadd_context_v_1(
) -> *mut wire_cst_context_v_1 {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(wire_cst_context_v_1::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_cst_new_box_autoadd_context_v_2(
) -> *mut wire_cst_context_v_2 {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(wire_cst_context_v_2::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_cst_new_box_autoadd_headers() -> *mut wire_cst_headers {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(wire_cst_headers::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_cst_new_box_autoadd_request_builder(
) -> *mut wire_cst_request_builder {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(
        wire_cst_request_builder::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_cst_new_box_autoadd_request_context(
) -> *mut wire_cst_request_context {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(
        wire_cst_request_context::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_cst_new_box_autoadd_u_64(value: u64) -> *mut u64 {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_cst_new_box_autoadd_u_8(value: u8) -> *mut u8 {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_cst_new_box_autoadd_unchecked_proposal(
) -> *mut wire_cst_unchecked_proposal {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(
        wire_cst_unchecked_proposal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_cst_new_box_autoadd_uri() -> *mut wire_cst_uri {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(wire_cst_uri::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_cst_new_box_autoadd_url() -> *mut wire_cst_url {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(wire_cst_url::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_cst_new_list_prim_u_8_loose(
    len: i32,
) -> *mut wire_cst_list_prim_u_8_loose {
    let ans = wire_cst_list_prim_u_8_loose {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_cst_new_list_prim_u_8_strict(
    len: i32,
) -> *mut wire_cst_list_prim_u_8_strict {
    let ans = wire_cst_list_prim_u_8_strict {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn frbgen_payjoin_flutter_cst_new_list_record_string_string(
    len: i32,
) -> *mut wire_cst_list_record_string_string {
    let wrap = wire_cst_list_record_string_string {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(
            <wire_cst_record_string_string>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(wrap)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_context_v_1 {
    field0: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_context_v_2 {
    field0: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_headers {
    field0: *mut wire_cst_list_record_string_string,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_prim_u_8_loose {
    ptr: *mut u8,
    len: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_prim_u_8_strict {
    ptr: *mut u8,
    len: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_record_string_string {
    ptr: *mut wire_cst_record_string_string,
    len: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_maybe_inputs_owned {
    field0: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_payjoin_error {
    tag: i32,
    kind: PayjoinErrorKind,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PayjoinErrorKind {
    InvalidAddress: wire_cst_PayjoinError_InvalidAddress,
    InvalidScript: wire_cst_PayjoinError_InvalidScript,
    NetworkValidation: wire_cst_PayjoinError_NetworkValidation,
    PsbtParseError: wire_cst_PayjoinError_PsbtParseError,
    ResponseError: wire_cst_PayjoinError_ResponseError,
    RequestError: wire_cst_PayjoinError_RequestError,
    TransactionError: wire_cst_PayjoinError_TransactionError,
    ServerError: wire_cst_PayjoinError_ServerError,
    SelectionError: wire_cst_PayjoinError_SelectionError,
    CreateRequestError: wire_cst_PayjoinError_CreateRequestError,
    PjParseError: wire_cst_PayjoinError_PjParseError,
    PjNotSupported: wire_cst_PayjoinError_PjNotSupported,
    ValidationError: wire_cst_PayjoinError_ValidationError,
    V2Error: wire_cst_PayjoinError_V2Error,
    UnexpectedError: wire_cst_PayjoinError_UnexpectedError,
    OhttpError: wire_cst_PayjoinError_OhttpError,
    UrlError: wire_cst_PayjoinError_UrlError,
    nil__: (),
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PayjoinError_InvalidAddress {
    message: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PayjoinError_InvalidScript {
    message: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PayjoinError_NetworkValidation {
    message: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PayjoinError_PsbtParseError {
    message: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PayjoinError_ResponseError {
    message: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PayjoinError_RequestError {
    message: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PayjoinError_TransactionError {
    message: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PayjoinError_ServerError {
    message: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PayjoinError_SelectionError {
    message: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PayjoinError_CreateRequestError {
    message: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PayjoinError_PjParseError {
    message: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PayjoinError_PjNotSupported {
    message: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PayjoinError_ValidationError {
    message: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PayjoinError_V2Error {
    message: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PayjoinError_UnexpectedError {
    message: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PayjoinError_OhttpError {
    message: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PayjoinError_UrlError {
    message: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_record_string_string {
    field0: *mut wire_cst_list_prim_u_8_strict,
    field1: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_request {
    url: wire_cst_url,
    body: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_request_builder {
    field0: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_request_context {
    field0: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_request_context_v_1 {
    request: wire_cst_request,
    context_v1: wire_cst_context_v_1,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_request_context_v_2 {
    request: wire_cst_request,
    context_v2: wire_cst_context_v_2,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_unchecked_proposal {
    field0: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_uri {
    field0: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_url {
    field0: usize,
}
