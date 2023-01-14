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
  operations: [ Convert ]
}

/// Converts the input string to a result
operation Convert {
  input: String,
  output: String
}

