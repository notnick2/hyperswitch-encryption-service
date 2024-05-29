use masking::StrongSecret;

use crate::errors::{self, CustomResult};

pub(crate) mod aes256;

#[cfg(feature = "aws")]
pub(crate) mod kms;

#[async_trait::async_trait]
pub(crate) trait Crypto {
    async fn encrypt(
        &self,
        input: StrongSecret<Vec<u8>>,
    ) -> CustomResult<StrongSecret<Vec<u8>>, errors::CryptoError>;
    async fn decrypt(
        &self,
        input: StrongSecret<Vec<u8>>,
    ) -> CustomResult<StrongSecret<Vec<u8>>, errors::CryptoError>;
}
