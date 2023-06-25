use std::env;

// Port returns the port your HTTP server should listen on.
fn port() -> Option<String> {
    env::var("PORT").ok()
}

// Revision returns the name of the Cloud Run revision being run.
fn revision() -> Option<String> {
    env::var("K_REVISION").ok()
}

// Configuration returns the name of the Cloud Run configuration being run.
fn configuration() -> Option<String> {
    env::var("K_CONFIGURATION").ok()
}

// ServiceName returns the name of the Cloud Run service being run.
fn service_name() -> Option<String> {
    env::var("K_SERVICE").ok()
}
