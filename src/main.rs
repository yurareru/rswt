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

const PROMPT: &str = "[guest@localhost ~]$ ";

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
    context.insert("shell", &format!("{}{}", PROMPT, form.command));
    match command {
        "help" => {
            let commands = ["cat", "clear", "help", "ls", "neofetch"];
            let mut formatted_commands = String::new();
            for command in commands.iter() {
                formatted_commands.push_str(&format!("\n {}", command));
            }
            context.insert(
                "res",
                &format!(
                    "These shell commands are defined internally.  Type 'help' to see this list.{}",
                    formatted_commands
                ),
            )
        }
        "clear" => {
            context.insert("res", "");
            return HttpResponse::NoContent().body(TEMPLATES.render("res.html", &context).unwrap());
        }
        "neofetch" => {
            let arch_linux_logo = r#"
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
            "#;
            let info = vec![
                ("OS", "Arch Linux x86_64"),
                ("Kernel", "Linux 7.2.70-arch1-1"),
                ("Uptime", "7 hours, 27 minutes"),
                ("Packages", "727 (pacman)"),
                ("Terminal", "Rust Web Terminal"),
                ("Terminal Font", "FiraCodeNF-Reg"),
                ("Theme", "Dark"),
                ("Locale", "en_US.UTF-8"),
            ];

            let colors_row1 = [
                "bg-[#000000]",
                "bg-[#a80000]",
                "bg-[#00a800]",
                "bg-[#a85400]",
                "bg-[#0000a8]",
                "bg-[#a800a8]",
                "bg-[#00a8a8]",
                "bg-[#a8a8a8]",
            ];

            let colors_row2 = [
                "bg-[#545454]",
                "bg-[#fc5454]",
                "bg-[#54fc54]",
                "bg-[#fcfc54]",
                "bg-[#5454fc]",
                "bg-[#f953f9]",
                "bg-[#54fcfc]",
                "bg-[#fcfcfc]",
            ];

            let mut formatted_info = String::new();
            for (key, value) in info {
                formatted_info.push_str(&format!(
                    "<span class=\"text-primary\">{}</span>: {}</br>",
                    key, value
                ));
            }
            let mut formatted_color_rows = String::new();
            for color in colors_row1.iter() {
                formatted_color_rows.push_str(&format!("<span class=\"{}\">   </span>", color));
            }
            formatted_color_rows.push_str("</br>");
            for color in colors_row2.iter() {
                formatted_color_rows.push_str(&format!("<span class=\"{}\">   </span>", color));
            }

            context.insert(
                "res",
                &format!(
                    "<div class=\"flex gap-[4ch]\">
<div class=\"text-primary\">{arch_linux_logo}</div>
<div><span class=\"text-primary\">guest</span>@<span class=\"text-primary\">localhost</span>
---------------
{formatted_info}

{formatted_color_rows}</div></div>",
                ),
            );
        }

        "ls" => context.insert("res", "orang.txt secret.txt"),
        cmd if cmd.starts_with("cat") => {
            let filename = cmd.trim_start_matches("cat").trim();
            match filename {
                "" => context.insert("res", ""),
                "orang.txt" => context.insert("res", "orang aring"),
                "secret.txt" => context.insert(
                    "res",
                    "cat: secret.txt: Permission denied<!-- I love Aya -->",
                ),
                other => {
                    context.insert("res", &format!("cat: {}: No such file or directory", other))
                }
            }
        }
        other => context.insert(
            "res",
            &format!(
                "{}: command not found\nType 'help' for a list of available commands.",
                other.split_whitespace().next().unwrap()
            ),
        ),
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
