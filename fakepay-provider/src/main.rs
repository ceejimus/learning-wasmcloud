//! Fakepay - stub payments capability provider
//!
use std::time::{SystemTime, UNIX_EPOCH};

use wasmbus_rpc::provider::prelude::*;
use wasmcloud_examples_payments::*;

const PAYMENT_METHODS: [(&str, &str); 2] = [
    (
        "Empire Credits",
        "aJh=h2uJmE6jqjcZgq3e9Rqgv7gORvlvCxTA6/hk1K/yIVmbq-nh!QCD8gTPs9dC",
    ),
    (
        "Hutt Loan",
        "s2zEL5qlFlLP/9x!Helow8hkDnKV0dpM3ykC?Ngt6cV7o3rI3aFkoP2?KfaAv=ve",
    ),
];

const VALID_AUTH_CODE: &str = "These are the products you're looking for.";

// Get some owned payment methods on the heap
fn payment_methods() -> impl IntoIterator<Item = PaymentMethod> {
    PAYMENT_METHODS
        .iter()
        .map(|(description, token)| PaymentMethod {
            description: Some(String::from(*description)),
            token: Some(String::from(*token)),
        })
        .collect::<Vec<PaymentMethod>>()
}

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
        arg: &AuthorizePaymentRequest,
    ) -> RpcResult<AuthorizePaymentResponse> {
        if payment_methods()
            .into_iter()
            .any(|payment_method| payment_method.token.unwrap() == arg.payment_method)
        {
            Ok(AuthorizePaymentResponse {
                auth_code: Some(String::from(VALID_AUTH_CODE)),
                fail_reason: None,
                success: true,
            })
        } else {
            Ok(AuthorizePaymentResponse {
                auth_code: None,
                fail_reason: Some(format!("Invalid payment method.")),
                success: false,
            })
        }
    }

    /// Completes a previously authorized payment.
    /// This operation requires the "authorization code" from a successful
    /// authorization operation.
    async fn complete_payment(
        &self,
        _ctx: &Context,
        arg: &CompletePaymentRequest,
    ) -> RpcResult<CompletePaymentResponse> {
        let start = SystemTime::now();
        let timestamp = start.duration_since(UNIX_EPOCH).unwrap().as_millis();
        if arg.auth_code == VALID_AUTH_CODE {
            Ok(CompletePaymentResponse {
                success: true,
                timestamp: timestamp as u64,
                txid: String::from("TxID"),
            })
        } else {
            Ok(CompletePaymentResponse {
                success: false,
                timestamp: timestamp as u64,
                txid: String::from("TxID"),
            })
        }
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
        Ok(Vec::from_iter(payment_methods()))
    }
}
