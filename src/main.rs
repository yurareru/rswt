use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use serde::Deserialize;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        }
    };
}

#[derive(Deserialize)]
struct Command {
    command: String,
}

const PROMPT: &str = "[guest@localhost ~]$";

#[get("/")]
async fn index() -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("shell", PROMPT);
    HttpResponse::Ok().body(TEMPLATES.render("index.html", &context).unwrap())
}

#[post("/command")]
async fn command(form: web::Form<Command>) -> impl Responder {
    let mut context = tera::Context::new();
    let command = form.command.trim();
    context.insert("shell", &format!("{}{}", PROMPT, command));
    match command {
        "help" => {
            context.insert("res", "orang help\n");
        }
        "clear" => {
            context.insert("res", "");
            return HttpResponse::NoContent().body(TEMPLATES.render("res.html", &context).unwrap());
        }
        "neofetch" => {
            context.insert(
                "res",
                "<pre>
                    -@
                   .##@
                  .####@
                  @#####@
                . *######@
               .##@o@#####@
              /############@
             /##############@
            @######@**%######@
           @######`     %#####o
          @######@       ######%
        -@#######h       ######@.`
       /#####h**``       `**%@####@
      @H@*`                    `*%#@
     *`                            `*
                </pre>",
            );
            return HttpResponse::Ok().body(TEMPLATES.render("neofetch.html", &context).unwrap());
        }
        other => {
            context.insert(
                "res",
                &format!(
                    "command not found: {}",
                    other.split_whitespace().next().unwrap()
                ),
            );
        }
    }

    HttpResponse::Ok().body(TEMPLATES.render("res.html", &context).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(command)
            .service(fs::Files::new("/", "./public"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
