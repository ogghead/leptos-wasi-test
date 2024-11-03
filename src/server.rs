use leptos::config::get_configuration;
use leptos::task::Executor;
use leptos_wasi::prelude::WasiExecutor;
use leptos_wasi::prelude::{IncomingRequest, ResponseOutparam};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::proxy::export;

use crate::app::{shell, App, SaveCount};

struct LeptosServer;

// NB(raskyld): for now, the types to use for the HTTP handlers are the one from
// the `leptos_wasi` crate, not the one generated in your crate.
impl Guest for LeptosServer {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        // Initiate a single-threaded [`Future`] Executor so we can run the
        // rendering system and take advantage of bodies streaming.
        let executor = WasiExecutor::new(leptos_wasi::executor::Mode::Stalled);
        Executor::init_local_custom_executor(executor.clone())
            .expect("cannot init future executor");
        executor.run_until(async {
            handle_request(request, response_out).await;
        })
    }
}

async fn handle_request(request: IncomingRequest, response_out: ResponseOutparam) {
    use leptos_wasi::prelude::Handler;

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;

    Handler::build(request, response_out)
        .expect("could not create handler")
        .with_server_fn::<SaveCount>()
        // Fetch all available routes from your App.
        .generate_routes(App)
        // Actually process the request and write the response.
        .handle_with_context(move || shell(leptos_options.clone()), || {})
        .await
        .expect("could not handle the request");
}

export!(LeptosServer with_types_in wasi);
