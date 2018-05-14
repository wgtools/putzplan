#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate chrono;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

use rocket_contrib::Template;

use chrono::prelude::*;

struct Job {
    name: String,
    condition: Box<Fn(usize) -> bool>,
    exclusion: Box<Fn(usize, String) -> bool>,
}

#[derive(Serialize)]
struct Week {
    num: usize,
    jobs: Vec<JobDTO>,
}

#[derive(Serialize, Clone)]
struct JobDTO(String);

#[derive(Serialize)]
struct Context {
    names: Vec<String>,
    weeks: Vec<Week>,
}

#[get("/")]
fn index() -> Template {
    let context = get_calendar();
    Template::render("index", &context)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, files])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}

/*
gets executed by every request, so no state has to be maintained
*/
fn get_calendar() -> Context {
    /*
    Vec of strings of names of people
    */
    let people: Vec<String> = vec!["sina", "marco", "kiv", "viki", "dietz", "alena"]
        .iter()
        .map(|i| i.to_string())
        .collect();

    /*
    The inividual jobs with their "todo-condition" and a optional exclusion
    */
    let bad = Job {
        name: "Bad".to_string(),
        condition: Box::new(|n| n % 2 == 0),
        exclusion: Box::new(|_, _| false),
    };

    let kuche = Job {
        name: "Küche".to_string(),
        condition: Box::new(|n| n % 2 == 0),
        exclusion: Box::new(|_, s| s == "marius"),
    };

    let flur = Job {
        name: "Flur".to_string(),
        condition: Box::new(|n| n % 1 == 0), //well...
        exclusion: Box::new(|_, _| false),
    };

    let jobs = vec![bad, kuche, flur];

    let week_num = Local::now().iso_week().week() as usize;
    let mut week_vec = Vec::new();
    let preview = 50;

    /*
    Determine what has to be done by whom based on the week number
    */
    for offset in 0..preview {
        let num_of_people = people.len();
        let mut job_vec = vec![JobDTO("-".to_string()); num_of_people];
        let calculated_week = (week_num + offset) % 53;

        let active_jobs: Vec<&Job> = jobs.iter()
            .filter(|i| (i.condition)(calculated_week))
            .map(|a| a)
            .collect();

        let num_of_things_to_do = active_jobs.len();
        let startindex = calculated_week % num_of_people;

        for i in startindex..(startindex + num_of_things_to_do) {
            let idx = (i % num_of_people) as usize;
            let person = &people[idx];
            let job = active_jobs[i - startindex];
            let dto = match (job.exclusion)(calculated_week, person.to_string()) {
                false => JobDTO(job.name.clone()),
                true => JobDTO("-".to_string()),
            };
            job_vec[idx] = dto;
        }

        let week = Week {
            num: calculated_week,
            jobs: job_vec,
        };
        week_vec.push(week);
    }

    /*
    // this can used to generate the "table" on the console
    print!("\t");
    people.iter().for_each(|p|print!("{}\t",p));
    println!(" ");
    week_vec.iter().for_each(|w|{
        print!("KW {}\t",w.num);
        w.jobs.iter().for_each(|j|{
            print!("{}\t",j.0);
        });
        println!(" ");
    });
    */

    Context {
        names: people,
        weeks: week_vec,
    }
}
