use rusqlite::Connection;
use scraper::Html;
use scraper::Selector;
use std::collections::HashMap;
use std::fmt::format;
use tokio::fs::File;

mod ids;
use ids::LegislaturaXV;

#[derive(Debug)]
struct Presence {
    date: String,
    name: String,
    party: String,
    presence: bool,
}

enum StateMachine {
    Name,
    Party,
    Presence,
    None,
}

async fn job(id: String) -> Vec<Presence> {
    let url = format!(
        "https://www.parlamento.pt/DeputadoGP/Paginas/DetalheReuniaoPlenaria.aspx?BID={}",
        id
    );
    let resp = reqwest::get(url).await.unwrap().text().await;

    if resp.is_err() {
        return Vec::new();
    }

    let resp = resp.unwrap();

    let document = Html::parse_document(&resp);
    let selector = Selector::parse(".row.margin_h0.margin-Top-15").unwrap();
    let selector_children = Selector::parse(".col-xs-12").unwrap();
    let selector_text = Selector::parse(".TextoRegular").unwrap();

    let date = document.select(&selector_text).next().unwrap().inner_html();
    let (_, date) = date.rsplit_once(' ').unwrap();

    let date = date[..date.len() - 1].to_string();

    println!("{:?}", date);

    let mut total = Vec::new();
    let rows = document.select(&selector);
    let mut curr_state = StateMachine::Name;
    for row in rows {
        let children = row.select(&selector_children);
        let mut name = String::new();
        let mut party = String::new();
        let mut presence = false;
        for child in children {
            let text = child.select(&selector_text).next().unwrap().inner_html();
            match curr_state {
                StateMachine::Name => {
                    name = text;
                    curr_state = StateMachine::Party;
                }
                StateMachine::Party => {
                    party = text;
                    curr_state = StateMachine::Presence;
                }
                StateMachine::Presence => {
                    presence = text.chars().nth(0).unwrap() == 'P';
                    curr_state = StateMachine::None;
                }
                StateMachine::None => {
                    let entry = Presence {
                        date: date.clone(),
                        name: name.clone(),
                        party: party.clone(),
                        presence,
                    };

                    total.push(entry);
                    curr_state = StateMachine::Name;
                }
            }
        }
    }
    total
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut to_add = Vec::new();
    let mut totals = Vec::new();
    let mut wow = 0;
    for n in LegislaturaXV {
        println!("Launching {wow}");
        wow += 1;
        let result = tokio::spawn(async move { job(n.to_string()).await });
        totals.push(result);
    }

    for t in totals {
        let mut demo = t.await?;
        to_add.append(&mut demo);
    }

    println!("Preparing to insert");

    let conn = Connection::open("./falta.db")?;

    conn.execute(
        "CREATE TABLE registry (
            id          INTEGER PRIMARY KEY,
            date        TEXT NOT NULL,
            name        TEXT NOT NULL,
            party       TEXT NOT NULL,
            presence    INTEGER NOT NULL
        )",
        (), // empty list of parameters.
    )?;

    for i in to_add {
        
        conn.execute(
            "INSERT INTO registry (date, name, party, presence) VALUES (?1, ?2, ?3, ?4)",
            (&i.date, &i.name, &i.party, &i.presence),
        )?;
    }
    println!("Done");

    Ok(())
}
