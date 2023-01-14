use orders::*;
use wasmcloud_examples_payments::*;
use wasmbus_rpc::actor::prelude::*;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Orders)]
struct BuyActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl Orders for BuyActor {
    async fn convert<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        _ctx: &Context,
        arg: &TS,
    ) -> RpcResult<String> {
        let provider = PaymentsSender::new();
        let payment_methods = provider.get_payment_methods(_ctx).await;

        let input = arg.to_string();
        let output = format!("Payment Methods: {:?}", payment_methods);
        Ok(output)
    }
}
