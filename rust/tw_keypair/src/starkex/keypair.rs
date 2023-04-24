// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use crate::starkex::private::PrivateKey;
use crate::starkex::public::PublicKey;
use crate::starkex::signature::Signature;
use crate::traits::{KeyPairTrait, SigningKeyTrait, VerifyingKeyTrait};
use crate::Error;
use tw_encoding::hex;

pub struct KeyPair {
    private: PrivateKey,
    public: PublicKey,
}

impl KeyPairTrait for KeyPair {
    type Private = PrivateKey;
    type Public = PublicKey;

    fn public(&self) -> &Self::Public {
        &self.public
    }

    fn private(&self) -> &Self::Private {
        &self.private
    }
}

impl SigningKeyTrait for KeyPair {
    type SigningHash = Vec<u8>;
    type Signature = Signature;

    fn sign(&self, hash: Self::SigningHash) -> Result<Self::Signature, Error> {
        self.private.sign(hash)
    }
}

impl VerifyingKeyTrait for KeyPair {
    type SigningHash = Vec<u8>;
    type VerifySignature = Signature;

    fn verify(&self, signature: Self::VerifySignature, hash: Self::SigningHash) -> bool {
        self.public.verify(signature, hash)
    }
}

impl<'a> TryFrom<&'a [u8]> for KeyPair {
    type Error = Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let private = PrivateKey::try_from(bytes)?;
        let public = private.public();
        Ok(KeyPair { private, public })
    }
}

impl From<&'static str> for KeyPair {
    fn from(hex: &'static str) -> Self {
        let bytes = hex::decode(hex).expect("Expected a valid Secret Key hex");
        KeyPair::try_from(bytes.as_slice()).expect("Expected a valid Secret Key")
    }
}