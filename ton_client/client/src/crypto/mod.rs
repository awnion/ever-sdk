/*
* Copyright 2018-2020 TON DEV SOLUTIONS LTD.
*
* Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
* this file except in compliance with the License.
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific TON DEV software governing permissions and
* limitations under the License.
*/

mod boxes;
mod errors;
mod hash;
mod hdkey;
pub(crate) mod internal;
mod keys;
mod math;
mod mnemonic;
mod nacl;
mod scrypt;

pub use boxes::SigningBoxHandle;
pub use errors::{Error, ErrorCode};
pub use keys::KeyPair;
pub use nacl::{
    ParamsOfNaclBox, ParamsOfNaclBoxKeyPairFromSecret, ParamsOfNaclBoxOpen, ParamsOfNaclSecretBox,
    ParamsOfNaclSecretBoxOpen, ParamsOfNaclSign, ParamsOfNaclSignDetached,
    ParamsOfNaclSignKeyPairFromSecret, ParamsOfNaclSignOpen, ResultOfNaclBox, ResultOfNaclBoxOpen,
    ResultOfNaclSign, ResultOfNaclSignDetached, ResultOfNaclSignOpen,
};

#[cfg(test)]
mod tests;

pub use crate::crypto::hash::{sha256, sha512};
pub use crate::crypto::hdkey::{
    hdkey_derive_from_xprv, hdkey_derive_from_xprv_path, hdkey_public_from_xprv,
    hdkey_secret_from_xprv, hdkey_xprv_from_mnemonic,
};
pub use crate::crypto::keys::{
    convert_public_key_to_ton_safe_format, generate_random_sign_keys, sign, verify_signature,
};
pub use crate::crypto::math::{factorize, generate_random_bytes, modular_power, ton_crc16};
pub use crate::crypto::mnemonic::{
    mnemonic_derive_sign_keys, mnemonic_from_entropy, mnemonic_from_random, mnemonic_verify,
    mnemonic_words,
};
pub use crate::crypto::nacl::{
    nacl_box, nacl_box_keypair, nacl_box_keypair_from_secret_key, nacl_box_open, nacl_secret_box,
    nacl_secret_box_open, nacl_sign, nacl_sign_detached, nacl_sign_keypair_from_secret_key,
    nacl_sign_open,
};
use crate::dispatch::{ModuleReg, Registrar};

pub(crate) const DEFAULT_MNEMONIC_DICTIONARY: u8 = 1;
pub(crate) const DEFAULT_MNEMONIC_WORD_COUNT: u8 = 12;
pub(crate) const DEFAULT_HDKEY_DERIVATION_PATH: &str = "m/44'/396'/0'/0/0";
pub(crate) const DEFAULT_HDKEY_COMPLIANT: bool = true;

/// Crypto functions.
#[derive(ApiModule)]
#[api_module(name = "crypto")]
pub(crate) struct CryptoModule;

impl ModuleReg for CryptoModule {
    fn reg(reg: &mut Registrar) {
        // Math

        reg.f(factorize, math::factorize_api);
        reg.f(modular_power, math::modular_power_api);
        reg.f(ton_crc16, math::ton_crc16_api);
        reg.f(generate_random_bytes, math::generate_random_bytes_api);

        // Keys

        reg.f(
            convert_public_key_to_ton_safe_format,
            keys::convert_public_key_to_ton_safe_format_api,
        );

        reg.f_no_args(
            generate_random_sign_keys,
            keys::generate_random_sign_keys_api,
        );
        reg.f(sign, keys::sign_api);
        reg.f(verify_signature, keys::verify_signature_api);

        // Sha

        reg.f(sha256, hash::sha256_api);
        reg.f(sha512, hash::sha512_api);

        // Scrypt

        reg.f(scrypt::scrypt, scrypt::scrypt_api);

        // NaCl

        reg.f(
            nacl_sign_keypair_from_secret_key,
            nacl::nacl_sign_keypair_from_secret_key_api,
        );
        reg.f(nacl_sign, nacl::nacl_sign_api);
        reg.f(nacl_sign_open, nacl::nacl_sign_open_api);
        reg.f(nacl_sign_detached, nacl::nacl_sign_detached_api);

        reg.f_no_args(nacl_box_keypair, nacl::nacl_box_keypair_api);
        reg.f(
            nacl_box_keypair_from_secret_key,
            nacl::nacl_box_keypair_from_secret_key_api,
        );
        reg.f(nacl_box, nacl::nacl_box_api);
        reg.f(nacl_box_open, nacl::nacl_box_open_api);
        reg.f(nacl_secret_box, nacl::nacl_secret_box_api);
        reg.f(nacl_secret_box_open, nacl::nacl_secret_box_open_api);

        // Mnemonic

        reg.f(mnemonic_words, mnemonic::mnemonic_words_api);
        reg.f(mnemonic_from_random, mnemonic::mnemonic_from_random_api);
        reg.f(mnemonic_from_entropy, mnemonic::mnemonic_from_entropy_api);
        reg.f(mnemonic_verify, mnemonic::mnemonic_verify_api);
        reg.f(
            mnemonic_derive_sign_keys,
            mnemonic::mnemonic_derive_sign_keys_api,
        );

        // HDKey

        reg.f(
            hdkey_xprv_from_mnemonic,
            hdkey::hdkey_xprv_from_mnemonic_api,
        );
        reg.f(hdkey_derive_from_xprv, hdkey::hdkey_derive_from_xprv_api);
        reg.f(
            hdkey_derive_from_xprv_path,
            hdkey::hdkey_derive_from_xprv_path_api,
        );
        reg.f(hdkey_secret_from_xprv, hdkey::hdkey_secret_from_xprv_api);
        reg.f(hdkey_public_from_xprv, hdkey::hdkey_public_from_xprv_api);
    }
}
