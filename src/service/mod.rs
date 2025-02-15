pub mod service {

    use polodb_core::{bson::doc, Collection, CollectionT, Database};
    use std::process;
    use whoami::username;

    use crate::model::model::{Id, Recap, Resultat, Tag, Tuto};

    pub fn establish_connection() -> Database {
        let db_path = format!("/home/{}/pwiz.db", username()); // chemin de la BDD
        let db = Database::open_path(&db_path).unwrap();
        db
    }

    pub fn get_resultats() -> Vec<Resultat> {
        let db: Database = establish_connection();
        let tutos: Collection<Tuto> = db.collection("tutos");
        let tags: Collection<Tag> = db.collection("tags");

        let mut resultats: Vec<Resultat> = Vec::new(); // Vec qui contiendra nos résultats de recherche

        let tags_result = tags.find(doc! {}).run();
        match tags_result {
            Ok(tags) => {
                //
                for tag in tags {
                    match tag {
                        Ok(tag) => {
                            //
                            let tuto_result = tutos
                                .find_one(doc! {"id": {"$eq": tag.tuto_id as i32} })
                                .unwrap();
                            match tuto_result {
                                Some(tuto) => {
                                    //
                                    let index =
                                        resultats[0..].iter().position(|x| x.tuto_id == tuto.id);

                                    if let None = index {
                                        let res = Resultat {
                                            author: tuto.author,
                                            score: 1,
                                            tuto_id: tuto.id,
                                            tags: vec![tag.value],
                                            title: tuto.title,
                                            content_type: tuto.content_type,
                                            content: tuto.content,
                                        };
                                        resultats.push(res);
                                    } else if let Some(index) = index {
                                        resultats[index].score += 1;
                                        resultats[index].tags.push(tag.value);
                                    }
                                }
                                None => println!("Aucun résultat n'a pu etre trouvé"),
                            }
                        }
                        Err(e) => {
                            println!(
                                "Erreur survenue lors de la recherche de tags dans la BDD: {:?}",
                                e
                            );
                        }
                    }
                }

                // classement des résultats par score (nombre de tags trouvés)
                resultats.sort_by(|a, b| b.score.cmp(&a.score));
                return resultats;
            }
            Err(_) => resultats,
        }
    }

    pub fn get_tuto(tuto_id: i32) -> Resultat {
        let db: Database = establish_connection();
        let tutos: Collection<Tuto> = db.collection("tutos");
        let tags: Collection<Tag> = db.collection("tags");

        let tuto = tutos
            .find_one(doc! {
            "id": tuto_id as i32 })
            .unwrap();

        match tuto {
            Some(tuto) => {
                let mut resultat = Resultat {
                    author: tuto.author,
                    score: 1,
                    tuto_id: tuto.id,
                    tags: Vec::new(),
                    title: tuto.title,
                    content_type: tuto.content_type,
                    content: tuto.content,
                };
                let tags = tags
                    .find(doc! {
                    "tuto_id": tuto_id as i32 })
                    .run();
                match tags {
                    Ok(tags) => {
                        for tag in tags {
                            if let Ok(tag) = tag {
                                resultat.tags.push(tag.value);
                            } else {
                                continue;
                            }
                        }
                    }
                    Err(_) => println!(
                        "Une erreur est survenue lors de la récupération des tags de ce tutoriel"
                    ),
                }
                resultat
            }
            None => {
                println!("Aucun tuto trouvé");
                process::exit(1);
            }
        }
    }

    pub fn insert_tuto(recap: Recap) -> i32 {
        let db: Database = establish_connection();
        let ids: Collection<Id> = db.collection("id");
        let tutos: Collection<Tuto> = db.collection("tutos");
        let tags: Collection<Tag> = db.collection("tags");

        let id = ids.find_one(doc! {}).unwrap();
        let tuto_id: i32;
        match id {
            Some(id) => {
                tuto_id = &id.value + 1;
                ids.update_one(
                    doc! {"value":&id.value},
                    doc! {
                        "$set":{
                            "value":&tuto_id,
                        }
                    },
                )
                .unwrap();
            }
            None => {
                tuto_id = 1;
                ids.insert_one(Id { value: tuto_id }).unwrap();
            }
        }

        if let Ok(_) = tutos.insert_one(Tuto {
            author: username(),
            id: tuto_id,
            title: recap.title,
            content_type: recap.content_type,
            content: recap.content,
        }) {
            let docs = recap
                .tags
                .into_iter()
                .map(|term| Tag {
                    tuto_id,
                    value: term,
                })
                .collect::<Vec<_>>();

            if let Ok(_) = tags.insert_many(docs) {
                tuto_id
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn update_tuto(id: i32, recap: &Recap) -> () {
        let db: Database = establish_connection();
        let tutos: Collection<Tuto> = db.collection("tutos");
        let tags: Collection<Tag> = db.collection("tags");

        tags.delete_many(doc! {"tuto_id":{"$eq":id}}).unwrap();
        //
        let updated = tutos.update_one(
            doc! {
                "id":id.to_owned() as i32
            },
            doc! {
                "$set":{
                    "title":&recap.title,
                    "content":&recap.content
                }
            },
        );
        match updated {
            Ok(_) => println!("\nTutoriel ayant pour index [{}] mis à jour:", id),
            Err(e) => println!("Erreur: {}", e),
        }

        tags.insert_many(
            recap
                .tags
                .iter()
                .filter(|term| term.to_string().cmp(&"".to_string()).is_ne())
                .map(|term| Tag {
                    tuto_id: id,
                    value: term.clone(),
                })
                .collect::<Vec<_>>(),
        )
        .unwrap();
    }

    pub fn delete_tuto(id: i32) {
        let db: Database = establish_connection();
        let tutos: Collection<Tuto> = db.collection("tutos");
        let tags: Collection<Tag> = db.collection("tags");

        tags.delete_many(doc! {"tuto_id":{"$eq":id}}).unwrap();
        tutos.delete_one(doc! {"id":{"$eq":id}}).unwrap();
    }

    pub fn appliquer_reglages(args: Vec<&String>) -> () {
        let db: Database = establish_connection();
        let tutos: Collection<Tuto> = db.collection("tutos");
        let tags: Collection<Tag> = db.collection("tags");
        let ids: Collection<Id> = db.collection("id");

        let mut reglages: Vec<String> = Vec::new();
        for arg in args {
            match arg.as_str() {
                "-d" => {
                    tutos.drop().unwrap();
                    tags.drop().unwrap();
                    ids.drop().unwrap();
                    reglages.push(String::from("suppression des collections"));
                }
                _ => println!("Option non reconnue: {}", arg),
            }
        }
        println!("Reglages appliqués: {:?}", reglages);
    }
}
