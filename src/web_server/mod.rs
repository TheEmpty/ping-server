use crate::server::LastSeenMap;
use chrono::Utc;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use std::{collections::HashMap, sync::Arc};

#[get("/metrics")]
async fn metrics(clients: &State<Arc<LastSeenMap>>) -> Template {
    let clients = clients.lock().await;
    let transformed: Vec<HashMap<&str, String>> = clients
        .iter()
        .map(|(name, last_seen_utc)| {
            HashMap::from([
                ("name", name.clone()),
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
