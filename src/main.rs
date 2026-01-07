// src/main.rs

use actix_multipart::Multipart;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use futures::{StreamExt, TryStreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::time::Duration;
use tera::{Context, Tera};
use uuid::Uuid;
use ammonia::Builder as AmmoniaBuilder;
use dotext::MsDoc;

// ----------------------------
// Estructuras OpenAI
// ----------------------------
#[derive(Serialize, Clone)]
struct InputMessage { role: String, content: String }

#[derive(Serialize)]
struct ResponsesRequest { model: String, input: Vec<InputMessage>, temperature: f32, store: bool }

#[derive(Deserialize, Default)]
struct ResponsesResponse { output: Vec<OutputItem> }

#[derive(Deserialize, Default)]
struct OutputItem { #[serde(rename = "type")] item_type: String, content: Option<Vec<ContentPart>> }

#[derive(Deserialize, Default)]
struct ContentPart { #[serde(rename = "type")] part_type: String, text: Option<String> }

fn extract_output_text(resp: &ResponsesResponse) -> Option<String> {
    for item in &resp.output {
        if item.item_type == "message" {
            if let Some(parts) = &item.content {
                for p in parts {
                    if p.part_type == "output_text" {
                        return p.text.clone();
                    }
                }
            }
        }
    }
    None
}

// ----------------------------
// PROMPTS DINÁMICOS (Multilingüe)
// ----------------------------

fn get_system_prompt(lang: &str) -> String {
    if lang == "ca" {
        return r#"
# ROL DEL SISTEMA
Actua como a acompanyant espiritual pastoral catòlic. El teu to és serè, acollidor i profund.
Evita el llenguatge tècnic, el judici moralitzant i les tradicions alienes al cristianisme.
RESPON ÚNICAMENT EN CATALÀ.

# OBJECTIU
Oferir una lectura espiritual de l'experiència de l'usuari que aporti pau i sentit.

# ESTRUCTURA DE SORTIDA (HTML OBLIGATORI)
Genera NOMÉS el contingut HTML dins d'etiquetes <article>.
Estructura exacta:

<article>
    <section class="acogida">
        <h2>1. Acollida i Lectura Espiritual</h2>
        <p>[Valida la vivència de l'usuari amb empatia i profunditat]</p>
    </section>

    <section class="sentido">
        <h2>2. Sentit Espiritual</h2>
        <p>[Discerniment: no és càstig, és camí, purificació o misteri]</p>
    </section>

    <section class="orientacion">
        <h2>3. Orientació Pastoral</h2>
        <p>[Consells senzills: confiança, pregària, paciència]</p>
    </section>

    <section class="cierre-biblico">
        <h2>4. Paraula de Vida</h2>
        <p>[Breu frase d'esperança]</p>
        <blockquote class="bible-quote">
            "[Cita bíblica explícita en català]"
            <footer>— <cite>[Llibre Cap:Ver]</cite></footer>
        </blockquote>
    </section>
</article>
"#.to_string();
    } else {
        return r#"
# ROL DEL SISTEMA
Actúa como un acompañante espiritual pastoral católico. Tu tono es sereno, acogedor y profundo.
Evita el lenguaje técnico, el juicio moralizante y las tradiciones ajenas al cristianismo.
RESPONDE ÚNICAMENTE EN ESPAÑOL.

# OBJETIVO
Ofrecer una lectura espiritual de la experiencia del usuario que aporte paz y sentido.

# ESTRUCTURA DE SALIDA (HTML OBLIGATORIO)
Genera SOLO el contenido HTML dentro de etiquetas <article>.
Estructura exacta:

<article>
    <section class="acogida">
        <h2>1. Acogida y Lectura Espiritual</h2>
        <p>[Valida la vivencia del usuario con empatia y profundidad]</p>
    </section>

    <section class="sentido">
        <h2>2. Sentido Espiritual</h2>
        <p>[Discernimiento: no es castigo, es camino, purificación o misterio]</p>
    </section>

    <section class="orientacion">
        <h2>3. Orientación Pastoral</h2>
        <p>[Consejos sencillos: confianza, oración, paciencia]</p>
    </section>

    <section class="cierre-biblico">
        <h2>4. Palabra de Vida</h2>
        <p>[Breve frase de esperanza]</p>
        <blockquote class="bible-quote">
            "[Cita bíblica explícita]"
            <footer>— <cite>[Libro Cap:Ver]</cite></footer>
        </blockquote>
    </section>
</article>
"#.to_string();
    }
}

// ----------------------------
// Utils
// ----------------------------

fn extract_text_from_file(filepath: &str, extension: &str) -> String {
    match extension {
        "txt" | "md" => fs::read_to_string(filepath).unwrap_or_default(),
        "docx" => {
            let mut content = String::new();
            if let Ok(mut doc) = dotext::Docx::open(filepath) { let _ = doc.read_to_string(&mut content); }
            content
        }
        "pdf" => pdf_extract::extract_text(filepath).unwrap_or_default(),
        _ => String::new(),
    }
}

fn sanitize_ai_html(html: &str) -> String {
    let tags: HashSet<&str> = ["article", "section", "div", "h2", "h3", "p", "span", "strong", "em", "br", "blockquote", "footer", "cite", "ul", "li", "a"].into_iter().collect();
    let generic_attrs: HashSet<&str> = ["class"].into_iter().collect();
    let mut builder = AmmoniaBuilder::default();
    builder.tags(tags);
    builder.generic_attributes(generic_attrs);
    builder.clean(html).to_string()
}

// ----------------------------
// Rutas
// ----------------------------

#[get("/")]
async fn index(tera: web::Data<Tera>) -> impl Responder {
    let rendered = tera.render("index.html", &Context::new()).unwrap_or_default();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[post("/analyze")]
async fn analyze(mut payload: Multipart, tera: web::Data<Tera>, client: web::Data<Client>) -> impl Responder {
    let api_key = env::var("OPENAI_API_KEY").unwrap_or_default();
    
    let mut experience_text = String::new();
    let mut context_text = String::new();
    let mut lang = "es".to_string(); // Default español

    while let Ok(Some(mut field)) = payload.try_next().await {
        let cd = field.content_disposition();
        let name = cd.get_name().unwrap_or("").to_string();
        let filename = cd.get_filename().map(|s| s.to_string());

        if let Some(fname) = filename {
            if fname.is_empty() { continue; }
            let tmp_path = format!("/tmp/{}", Uuid::new_v4());
            let mut f = fs::File::create(&tmp_path).unwrap();
            while let Some(chunk) = field.next().await { f.write_all(&chunk.unwrap()).unwrap(); }
            
            let ext = fname.rsplit('.').next().unwrap_or("").to_lowercase();
            let extracted = extract_text_from_file(&tmp_path, &ext);
            
            if name == "experience_file" { experience_text.push_str(&format!("\n[Adjunto: {}]\n{}", fname, extracted)); }
            else if name == "context_file" { context_text.push_str(&format!("\n[Adjunto: {}]\n{}", fname, extracted)); }
            let _ = fs::remove_file(&tmp_path);
        } else {
            let mut value = Vec::new();
            while let Some(chunk) = field.next().await { value.extend_from_slice(&chunk.unwrap()); }
            let text = String::from_utf8(value).unwrap_or_default();
            
            match name.as_str() {
                "experience" => experience_text.push_str(&text),
                "context" => context_text.push_str(&text),
                "lang" => lang = text, // Capturamos el idioma del formulario
                _ => {}
            }
        }
    }

    let user_message = format!("=== EXPERIENCIA ===\n{}\n\n=== CONTEXTO ===\n{}", experience_text, context_text);
    
    // Seleccionar prompt según idioma
    let system_prompt = get_system_prompt(&lang);

    let request_body = ResponsesRequest {
        model: env::var("AI_MODEL").unwrap_or("gpt-4o-mini".to_string()),
        input: vec![
            InputMessage { role: "system".to_string(), content: system_prompt },
            InputMessage { role: "user".to_string(), content: user_message },
        ],
        temperature: 0.7,
        store: false,
    };

    match client.post("https://api.openai.com/v1/responses").bearer_auth(&api_key).json(&request_body).send().await {
        Ok(r) => {
            let json: ResponsesResponse = r.json().await.unwrap_or_default();
            let content = extract_output_text(&json).unwrap_or("<p>...</p>".to_string());
            
            let mut ctx = Context::new();
            ctx.insert("report", &sanitize_ai_html(&content));
            // Pasamos el idioma al template para traducir los botones de salida
            ctx.insert("lang", &lang); 
            
            let rendered = tera.render("report.html", &ctx).unwrap_or_default();
            HttpResponse::Ok().content_type("text/html").body(rendered)
        },
        Err(_) => HttpResponse::InternalServerError().body("Error de conexión / Error de connexió"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let tera = Tera::new("templates/**/*").expect("Error templates");
    let client = Client::builder().timeout(Duration::from_secs(90)).build().unwrap();
    let port = env::var("PORT").unwrap_or("8080".to_string()).parse().unwrap();
    println!("Servidor Pastoral (Bilingüe) iniciado en puerto: {}", port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::PayloadConfig::new(20 * 1024 * 1024))
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(client.clone()))
            .service(index)
            .service(analyze)
    }).bind(("0.0.0.0", port))?.run().await
}

