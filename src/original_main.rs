use crate::{gen}
fn main() -> Result<(), Box<dyn Error> {
    generate()
}

// use std::error::Error;

// use lsp_types::OneOf;
// use lsp_types::{
//     request::GotoDefinition, GotoDefinitionResponse, InitializeParams, ServerCapabilities,
// };

// use lsp_server::{Connection, ExtractError, Message, Request, RequestId, Response};

// mod server_req;

// fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
//     eprintln!("starting Octave LSP server");

//     let (connection, io_threads) = Connection::stdio();

//     // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
//     let x = ServerCapabilities {
//         definition_provider: Some(OneOf::Left(true)),
//         ..Default::default()
//     };
//     let server_capabilities = serde_json::to_value(x).unwrap();
//     let initialization_params = connection.initialize(server_capabilities)?;
//     main_loop(connection, initialization_params)?;
//     io_threads.join()?;

//     // Shut down gracefully.
//     eprintln!("shutting down server");
//     Ok(())
// }

// fn main_loop(
//     connection: Connection,
//     params: serde_json::Value,
// ) -> Result<(), Box<dyn Error + Sync + Send>> {
//     let _params: InitializeParams = serde_json::from_value(params).unwrap();
//     eprintln!("starting main loop");
//     for msg in &connection.receiver {
//         eprintln!("got msg: {:?}", msg);
//         let server_req = server_req::ServerRequest { msg };
//         match server_req {
//             Message::Response(resp) => {
//                 eprintln!("got response: {:?}", resp);
//             }
//             Message::Notification(not) => {
//                 eprintln!("got notification: {:?}", not);
//             }
            
//             Message::Request(req) => {
//                 if connection.handle_shutdown(&req)? {
//                     return Ok(());
//                 }
//                 eprintln!("got request: {:?}", req);
//                 match cast::<GotoDefinition>(req) {
//                     Ok((id, params)) => {
//                         eprintln!("got gotoDefinition request #{}: {:?}", id, params);
//                         let result = Some(GotoDefinitionResponse::Array(Vec::new()));
//                         let result = serde_json::to_value(&result).unwrap();
//                         let resp = Response {
//                             id,
//                             result: Some(result),
//                             error: None,
//                         };
//                         connection.sender.send(Message::Response(resp))?;
//                         continue;
//                     }
//                     Err(err @ ExtractError::JsonError { .. }) => panic!("{:?}", err),
//                     Err(ExtractError::MethodMismatch(req)) => req,
//                 };
//                 // ...
//             }
//         }
//     }
//     Ok(())
// }

// fn cast<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
// where
//     R: lsp_types::request::Request,
//     R::Params: serde::de::DeserializeOwned,
// {
//     req.extract(R::METHOD)
// }
