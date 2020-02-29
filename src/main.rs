mod fetcher;
mod models;

use serde::{Deserialize, Serialize};
#[macro_use]
extern crate lazy_static;

use actix_web::middleware::cors::Cors;
use actix_web::{delete, get, post, put, web, App, Error, HttpResponse, HttpServer, Result};
use models::{AirportConfiguration, Configuration, GroupConfiguration};

lazy_static! {
    static ref FETCHER: fetcher::Fetcher = fetcher::Fetcher::new("data");
}

#[get("/health")]
async fn health() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(true))
}

#[put("/api/v1/configurations/{id}")]
async fn create_configuration(configuration_id: web::Path<String>) -> Result<HttpResponse, Error> {
    if let Some(_) = FETCHER.get_configuration(&configuration_id) {
        return Ok(HttpResponse::Conflict().json("Configuration already exists"));
    }
    let configuration = Configuration::empty(&configuration_id);
    FETCHER.save_configuration(&configuration);
    Ok(HttpResponse::Ok().json(configuration))
}

#[delete("/api/v1/configurations/{id}")]
async fn delete_configuration(configuration_id: web::Path<String>) -> Result<HttpResponse, Error> {
    match FETCHER.get_configuration(&configuration_id) {
        Some(configuration) => {
            FETCHER.delete_configuration(&configuration_id);
            Ok(HttpResponse::Ok().json(configuration))
        }
        None => Ok(HttpResponse::NotFound().json("Configuration Not Found")),
    }
}

#[get("/api/v1/configurations/{id}")]
async fn get_configuration(configuration_id: web::Path<String>) -> Result<HttpResponse, Error> {
    match FETCHER.get_configuration(&configuration_id) {
        Some(configuration) => Ok(HttpResponse::Ok().json(configuration)),
        None => Ok(HttpResponse::NotFound().json("Configuration Not Found")),
    }
}

#[post("/api/v1/configurations/{config_id}/airports")]
async fn upsert_airport_configuration(
    path: web::Path<String>,
    airport: web::Json<AirportConfiguration>,
) -> Result<HttpResponse, Error> {
    if let Some(mut configuration) = FETCHER.get_configuration(&path) {
        configuration.upsert_airport_configuration(airport.into_inner());
        FETCHER.save_configuration(&configuration);
        Ok(HttpResponse::Ok().json(configuration))
    } else {
        Ok(HttpResponse::NotFound().json("Configuration Not Found"))
    }
}

#[post("/api/v1/configurations/{config_id}/groups")]
async fn upsert_group_configuration(
    path: web::Path<String>,
    group: web::Json<GroupConfiguration>,
) -> Result<HttpResponse, Error> {
    if let Some(mut configuration) = FETCHER.get_configuration(&path) {
        configuration.upsert_group_configuration(group.into_inner());
        FETCHER.save_configuration(&configuration);
        Ok(HttpResponse::Ok().json(configuration))
    } else {
        Ok(HttpResponse::NotFound().json("Configuration Not Found"))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TextConfiguration {
    text: String,
}

#[post("/api/v1/configurations/{config_id}/rules")]
async fn update_rules_configuration(
    path: web::Path<String>,
    text: web::Json<TextConfiguration>,
) -> Result<HttpResponse, Error> {
    if let Some(mut configuration) = FETCHER.get_configuration(&path) {
        configuration.update_rules(&text.text);
        FETCHER.save_configuration(&configuration);
        Ok(HttpResponse::Ok().json(configuration))
    } else {
        Ok(HttpResponse::NotFound().json("Configuration Not Found"))
    }
}

#[post("/api/v1/configurations/{config_id}/generals")]
async fn update_generals_configuration(
    path: web::Path<String>,
    text: web::Json<TextConfiguration>,
) -> Result<HttpResponse, Error> {
    if let Some(mut configuration) = FETCHER.get_configuration(&path) {
        configuration.update_generals(&text.text);
        FETCHER.save_configuration(&configuration);
        Ok(HttpResponse::Ok().json(configuration))
    } else {
        Ok(HttpResponse::NotFound().json("Configuration Not Found"))
    }
}

#[post("/api/v1/configurations/{config_id}/url_rewrites")]
async fn update_url_rewrites_configuration(
    path: web::Path<String>,
    text: web::Json<TextConfiguration>,
) -> Result<HttpResponse, Error> {
    if let Some(mut configuration) = FETCHER.get_configuration(&path) {
        configuration.update_url_rewrites(&text.text);
        FETCHER.save_configuration(&configuration);
        Ok(HttpResponse::Ok().json(configuration))
    } else {
        Ok(HttpResponse::NotFound().json("Configuration Not Found"))
    }
}

#[get("/api/v1/configurations/{config_id}/surge")]
async fn get_surge_configurationpath(path: web::Path<String>) -> Result<HttpResponse, Error> {
    if let Some(configuration) = FETCHER.get_configuration(&path) {
        if let Some(surge_configuration) = configuration.fetch_surge_configuration().await {
            Ok(HttpResponse::Ok().body(surge_configuration.to_string()))
        } else {
            Ok(HttpResponse::BadRequest().json("Fail to generation surge configuration"))
        }
    } else {
        Ok(HttpResponse::NotFound().json("Configuration Not Found"))
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if let Err(_) = std::env::var("SERVER_HOST") {
        println!("Environment variable SERVER_HOST is not set.");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Environment variable SERVER_HOST is not set.",
        ));
    }

    let init_closure = || {
        App::new().configure(|app| {
            Cors::for_app(app)
                .service(health)
                .service(create_configuration)
                .service(get_configuration)
                .service(upsert_airport_configuration)
                .service(upsert_group_configuration)
                .service(update_rules_configuration)
                .service(update_generals_configuration)
                .service(update_url_rewrites_configuration)
                .service(get_surge_configurationpath)
        })
    };
    HttpServer::new(init_closure)
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
