use std::collections::HashMap;

use orders::*;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_examples_payments::*;

const PRODUCTS: [(&str, u32); 3] = [("speeder", 3000), ("blaster", 399), ("droid", 600)];

fn products() -> HashMap<&'static str, u32> {
    PRODUCTS
        .iter()
        .map(|(product, price)| (*product, *price))
        .collect::<HashMap<&str, u32>>()
}

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Orders)]
struct BuyActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl Orders for BuyActor {
    async fn purchase<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<String> {
        let products = products();

        let product = arg.to_string();

        let price = match products.get(&product[..]) {
            Some(price) => price,
            None => {
                return Err(RpcError::Other(format!(
                    "Product not in stock!!! Choose from: {:?}",
                    products
                )))
            }
        };

        let provider = PaymentsSender::new();
        let payment_method = provider
            .get_payment_methods(ctx)
            .await?
            .into_iter()
            .next()
            .unwrap();

        let request = AuthorizePaymentRequest {
            amount: 3000,
            payment_entity: String::from("PaymentEntity"),
            payment_method: payment_method.token.clone().unwrap(),
            reference_id: String::from("RefID"),
            tax: 30,
        };

        let response = provider.authorize_payment(ctx, &request).await?;

        if !response.success || response.auth_code.is_none() {
            return Err(RpcError::Other(format!("Request validation failed.")));
        }

        let request = CompletePaymentRequest {
            auth_code: response.auth_code.unwrap(),
            description: Some(format!("Purchasing a {} for {} credits.", product, price)),
        };

        let response = provider.complete_payment(ctx, &request).await?;

        if !response.success {
            return Err(RpcError::Other(format!("Failed to complete payment.")));
        }

        Ok(format!(
            "Congats! You are the proud owner of a new {}!",
            product
        ))
    }
}
