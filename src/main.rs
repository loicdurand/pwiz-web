pub mod model;
pub mod service;

#[macro_use]
extern crate rocket;
extern crate tera;

use whoami::username;

//imports ~ rocket + templates
use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::http::Status;
use rocket::response::{content, status, Redirect};
use rocket_dyn_templates::{context, Template};

#[derive(FromForm, Debug)]
pub struct Recap {
    // The raw, undecoded value. You _probably_ want `String` instead.
    title: String,
    content: String,
    tags: String,
}

#[get("/")]
fn index() -> content::RawHtml<Template> {
    //on retourne du HTML, généré par un template

    let resultats = service::service::get_resultats();

    content::RawHtml(Template::render(
        "index", //nom du template (index.tera)
        context! {
           resultats,
        }, //on passe en argument à notre template le résultat de notre requête sql
    ))
}

#[get("/afficher/<id>")]
pub fn afficher(id: i32) -> content::RawHtml<Template> {
    let resultat = service::service::get_tuto(id);

    content::RawHtml(Template::render(
        "afficher", //nom du template (index.tera)
        context! {
           resultat
        }, //on passe en argument à notre template le résultat de notre requête sql
    ))
}

#[get("/creer")]
pub fn creer() -> content::RawHtml<Template> {
    content::RawHtml(Template::render(
        "creer", //nom du template (index.tera)
        context! {},
    ))
}

#[post(
    "/creer",
    data = "<data>",
    format = "application/x-www-form-urlencoded"
)]
pub fn creer_submit(data: Form<Recap>) -> Redirect {
    let tags = data.tags.split(' ').collect::<Vec<&str>>();
    let mut content:Vec<String> = Vec::new();
    for line in data.content.lines() {
        content.push(line.trim().to_string());
    }

    let recap = model::model::Recap {
        author: username(),
        title: data.title.clone(),
        content_type: "commande".to_string(),
        content,
        tags: tags.iter().map(|s| s.to_string()).collect(),
    };

    service::service::insert_tuto(recap);

    Redirect::to(uri!(index))
}

#[get("/modifier/<id>")]
pub fn modifier(id: i32) -> content::RawHtml<Template> {
    
    let resultat = service::service::get_tuto(id);

    content::RawHtml(Template::render(
        "modifier", //nom du template (index.tera)
        context! {
           resultat
        }, //on passe en argument à notre template le résultat de notre requête sql
    ))
}

#[post(
    "/modifier/<id>",
    data = "<data>",
    format = "application/x-www-form-urlencoded"
)]
pub fn modifier_submit(id: i32, data: Form<Recap>) -> content::RawHtml<Template> {
    let tags = data.tags.split(' ').collect::<Vec<&str>>();

    let recap = model::model::Recap {
        author: username(),
        title: data.title.clone(),
        content_type: "commande".to_string(),
        content: vec![data.content.clone()],
        tags: tags.iter().map(|s| s.to_string()).collect(),
    };

    service::service::update_tuto(id, &recap);

    let resultat = service::service::get_tuto(id);

    content::RawHtml(Template::render(
        "modifier", //nom du template (index.tera)
        context! {
           resultat
        }, //on passe en argument à notre template le résultat de notre requête sql
    ))
}

#[delete("/supprimer/<id>")]
pub fn supprimer(id: i32) -> status::Custom<content::RawJson<&'static str>> {
    service::service::delete_tuto(id);

    status::Custom(
        Status::ImATeapot,
        content::RawJson("{ \"status\": \"ok\" }"),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                afficher,
                creer,
                creer_submit,
                modifier,
                modifier_submit,
                supprimer
            ],
        ) //on active notre route
        .mount("/public", FileServer::from(relative!("public"))) //on active le serveur de fichiers statiques
        .attach(Template::fairing()) //on ajoute le templating au cycle de vie de rocket
}