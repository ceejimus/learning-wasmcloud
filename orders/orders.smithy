// orders.smithy
//

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.wasmcloud.examples.orders", crate: "orders" } ]

namespace org.wasmcloud.examples.orders

use org.wasmcloud.model#wasmbus

/// Description of Orders service
@wasmbus( actorReceive: true )
service Orders {
  version: "0.1",
  operations: [ Purchase ]
}

/// Converts the input string to a result
operation Purchase {
  input: String,
  output: String
}

