use crate::server::LastSeenMap;
use chrono::Utc;
use lazy_static::lazy_static;
use regex::Regex;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use std::{collections::HashMap, sync::Arc};

lazy_static! {
    /// https://prometheus.io/docs/concepts/data_model/#metric-names-and-labels
    static ref NON_VALID_PROMETHEUS_CHARACTERS: Regex = Regex::new(r"[^a-zA-Z0-9_:]").unwrap();
}

#[get("/metrics")]
async fn metrics(clients: &State<Arc<LastSeenMap>>) -> Template {
    let clients = clients.lock().await;
    let transformed: Vec<HashMap<&str, String>> = clients
        .iter()
        .map(|(name, last_seen_utc)| {
            HashMap::from([
                ("name", sanitize_client_name(name)),
                (
                    "last_seen",
                    Utc::now()
                        .signed_duration_since(*last_seen_utc)
                        .num_seconds()
                        .to_string(),
                ),
            ])
        })
        .collect();
    let result = Template::render(
        "metrics",
        context! {
            clients: transformed,
        },
    );
    drop(clients);
    result
}

pub(crate) async fn launch(clients: Arc<LastSeenMap>) {
    let rocket = rocket::build()
        .mount("/", routes![metrics])
        .manage(clients)
        .attach(Template::fairing())
        .ignite()
        .await
        .expect("Failed to ignite")
        .launch();
    tokio::spawn(rocket);
}

fn sanitize_client_name(name: &str) -> String {
    let name = name.to_lowercase();
    let mut name = NON_VALID_PROMETHEUS_CHARACTERS
        .replace_all(&name, "_")
        .to_string();
    match name.as_bytes()[0] as char {
        '0'..='9' => {
            let num = name.remove(0);
            format!("{name}_{num}")
        }
        _ => name,
    }
}
