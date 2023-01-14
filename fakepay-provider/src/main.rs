//! Fakepay - stub payments capability provider
//!
use wasmbus_rpc::provider::prelude::*;
use wasmcloud_examples_payments::*;

// Start the provider and run until stopped by the host
fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(FakePayProvider::default(), Some("FakePay".to_string()))?;
    eprintln!("FakePay provider exiting");
    Ok(())
}

/// FakePay capability provider implementation
#[derive(Default, Clone, Provider)]
#[services(Payments)]
struct FakePayProvider {}

// use default implementations of provider message handlers
impl ProviderDispatch for FakePayProvider {}
impl ProviderHandler for FakePayProvider {}

/// Handle FakePay methods
#[async_trait]
impl Payments for FakePayProvider {
    /// AuthorizePayment - Validates that a potential payment transaction
    /// can go through. If this succeeds then we should assume it is safe
    /// to complete a payment. Payments _cannot_ be completed without getting
    /// a validation code (in other words, all payments have to be pre-authorized).
    async fn authorize_payment(
        &self,
        _ctx: &Context,
        _arg: &AuthorizePaymentRequest,
    ) -> RpcResult<AuthorizePaymentResponse> {
        Err(RpcError::NotImplemented)
    }

    /// Completes a previously authorized payment.
    /// This operation requires the "authorization code" from a successful
    /// authorization operation.
    async fn complete_payment(
        &self,
        _ctx: &Context,
        _arg: &CompletePaymentRequest,
    ) -> RpcResult<CompletePaymentResponse> {
        Err(RpcError::NotImplemented)
    }

    /// `GetPaymentMethods` - Retrieves an _opaque_ list of payment methods,
    /// which is a list of customer-facing method names and the
    /// _[tokens](https://en.wikipedia.org/wiki/Tokenization_(data_security))_
    /// belonging to that payment method. You could think of this list as
    /// a previously saved list of payment methods stored in a "wallet".
    /// A payment method _token_ is required to authorize and subsequently
    /// complete a payment transaction. A customer could have previously
    /// supplied their credit card and user-friendly labels for those methods
    /// like "personal" and "work", etc.
    async fn get_payment_methods(&self, _ctx: &Context) -> RpcResult<PaymentMethods> {
        Ok(vec![
           PaymentMethod {
            description: Some(String::from("Checking Account")),
            token: Some(String::from(
                "aJh=h2uJmE6jqjcZgq3e9Rqgv7gORvlvCxTA6/hk1K/yIVmbq-nh!QCD8gTPs9dC",
            )),
           },
           PaymentMethod {
            description: Some(String::from("Hip Crypto")),
            token: Some(String::from(
                "s2zEL5qlFlLP/9x!Helow8hkDnKV0dpM3ykC?Ngt6cV7o3rI3aFkoP2?KfaAv=ve",
            )),
           },
           PaymentMethod {
            description: Some(String::from("My VISA")),
            token: Some(String::from(
                "hDl8Ar6=6ypmZkrD1SLYSdWj1JSkeB3k6C-dflIEQ6byO371xVSWUsmqiLN5aUWQ",
            )),
           }
        ])
    }
}
