mod gui;

use std::io::{self, Write};
use std::process::Command;

const MAX_DOWNLOAD_BYTES: &str = "5242880";
const MAX_VISIBLE_CHARS: usize = 30_000;
const MAX_LINKS: usize = 250;
const META_MARKER: &[u8] = b"\n__RUSTY_BROWSER_META__";

#[derive(Clone, Debug)]
pub(crate) struct Link {
    pub(crate) label: String,
    pub(crate) url: String,
}

#[derive(Clone, Debug)]
pub(crate) struct Page {
    pub(crate) url: String,
    pub(crate) title: String,
    pub(crate) text: String,
    pub(crate) links: Vec<Link>,
}

#[derive(Default)]
struct Browser {
    history: Vec<Page>,
    position: Option<usize>,
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments
        .iter()
        .any(|argument| argument == "--help" || argument == "-h")
    {
        println!(
            "Rusty Browser\n\nUso: rusty-browser [--text]\n\nSenza opzioni apre la finestra grafica. --text avvia la versione testuale."
        );
        return;
    }
    if arguments.iter().any(|argument| argument == "--text") {
        run_text_browser();
        return;
    }
    if let Err(error) = gui::run() {
        eprintln!("Impossibile avviare la finestra grafica: {error}");
        eprintln!("Puoi usare la modalità testuale con: rusty-browser --text");
    }
}

fn run_text_browser() {
    println!("Rusty Browser — browser web testuale in Rust");
    println!("Inserisci un indirizzo, una ricerca o ? per i comandi.\n");

    let mut browser = Browser::default();
    let stdin = io::stdin();

    loop {
        print!("rusty> ");
        if io::stdout().flush().is_err() {
            break;
        }

        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(error) => {
                eprintln!("Errore di lettura: {error}");
                break;
            }
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        match input.to_ascii_lowercase().as_str() {
            "q" | "quit" | "exit" => break,
            "?" | "help" => print_help(),
            "b" | "back" => browser.back(),
            "f" | "forward" => browser.forward(),
            "r" | "reload" => browser.reload(),
            _ => {
                if let Ok(number) = input.parse::<usize>() {
                    browser.open_link(number);
                } else if let Some(number) = input.strip_prefix("open ") {
                    match number.trim().parse::<usize>() {
                        Ok(number) => browser.open_link(number),
                        Err(_) => println!("Scrivi il numero del link, per esempio: open 2"),
                    }
                } else {
                    browser.navigate(input);
                }
            }
        }
    }

    println!("A presto!");
}

impl Browser {
    fn navigate(&mut self, input: &str) {
        let target = make_target(input);
        println!("Carico {target} ...");
        match fetch_page(&target) {
            Ok(page) => {
                if let Some(position) = self.position {
                    self.history.truncate(position + 1);
                }
                self.history.push(page);
                self.position = Some(self.history.len() - 1);
                self.show_current();
            }
            Err(error) => eprintln!("Non riesco ad aprire la pagina: {error}"),
        }
    }

    fn open_link(&mut self, number: usize) {
        let Some(position) = self.position else {
            println!("Apri prima una pagina web.");
            return;
        };
        let Some(link) = self.history[position].links.get(number.saturating_sub(1)) else {
            println!("Numero di link non valido.");
            return;
        };
        let target = link.url.clone();
        self.navigate(&target);
    }

    fn back(&mut self) {
        match self.position {
            Some(position) if position > 0 => {
                self.position = Some(position - 1);
                self.show_current();
            }
            _ => println!("Non ci sono pagine precedenti."),
        }
    }

    fn forward(&mut self) {
        match self.position {
            Some(position) if position + 1 < self.history.len() => {
                self.position = Some(position + 1);
                self.show_current();
            }
            _ => println!("Non ci sono pagine successive."),
        }
    }

    fn reload(&mut self) {
        let Some(position) = self.position else {
            println!("Apri prima una pagina web.");
            return;
        };
        let url = self.history[position].url.clone();
        println!("Ricarico {url} ...");
        match fetch_page(&url) {
            Ok(page) => {
                self.history[position] = page;
                self.show_current();
            }
            Err(error) => eprintln!("Non riesco a ricaricare la pagina: {error}"),
        }
    }

    fn show_current(&self) {
        let Some(position) = self.position else {
            return;
        };
        let page = &self.history[position];
        println!("\n{}\n{}", page.title, page.url);
        println!("{}", "─".repeat(72));
        if page.text.is_empty() {
            println!("La pagina non contiene testo leggibile.");
        } else {
            let mut text = page.text.as_str();
            if page.text.chars().count() > MAX_VISIBLE_CHARS {
                let end = page
                    .text
                    .char_indices()
                    .nth(MAX_VISIBLE_CHARS)
                    .map(|(index, _)| index)
                    .unwrap_or(page.text.len());
                text = &page.text[..end];
            }
            println!("{}", text.trim());
            if page.text.chars().count() > MAX_VISIBLE_CHARS {
                println!("\n[Contenuto abbreviato per la visualizzazione]");
            }
        }

        if !page.links.is_empty() {
            println!("\nLink:");
            for (index, link) in page.links.iter().enumerate() {
                println!("  [{}] {} — {}", index + 1, link.label, link.url);
            }
        }
        println!(
            "\nComandi: numero per aprire un link · b indietro · f avanti · r ricarica · q esci\n"
        );
    }
}

fn print_help() {
    println!(
        "\nComandi disponibili:\n  https://example.com  apre un indirizzo\n  parole da cercare     cerca sul web\n  3 oppure open 3       apre il terzo link della pagina\n  b                     torna indietro\n  f                     vai avanti\n  r                     ricarica la pagina\n  q                     esce\n"
    );
}

pub(crate) fn make_target(input: &str) -> String {
    let trimmed = input.trim();
    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("https://") || lower.starts_with("http://") {
        return trimmed.to_owned();
    }
    if trimmed.contains(' ') || !trimmed.contains('.') {
        return format!(
            "https://html.duckduckgo.com/html/?q={}",
            percent_encode(trimmed)
        );
    }
    format!("https://{trimmed}")
}

fn percent_encode(value: &str) -> String {
    let mut result = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            result.push(byte as char);
        } else {
            result.push_str(&format!("%{byte:02X}"));
        }
    }
    result
}

pub(crate) fn fetch_page(url: &str) -> Result<Page, String> {
    let output = Command::new("curl")
        .args([
            "--location",
            "--fail",
            "--silent",
            "--show-error",
            "--max-time",
            "25",
            "--max-filesize",
            MAX_DOWNLOAD_BYTES,
            "--user-agent",
            "RustyBrowser/0.1 (text browser)",
            "--write-out",
            "\n__RUSTY_BROWSER_META__%{url_effective}\n%{http_code}\n%{content_type}",
            url,
        ])
        .output()
        .map_err(|error| format!("impossibile avviare curl ({error})"))?;

    let marker_position = output
        .stdout
        .windows(META_MARKER.len())
        .rposition(|window| window == META_MARKER)
        .ok_or_else(|| {
            let detail = String::from_utf8_lossy(&output.stderr).trim().to_owned();
            if detail.is_empty() {
                "risposta non valida dal server".to_owned()
            } else {
                detail
            }
        })?;
    let body = &output.stdout[..marker_position];
    let metadata = String::from_utf8_lossy(&output.stdout[marker_position + META_MARKER.len()..]);
    let mut metadata_lines = metadata.lines();
    let final_url = metadata_lines.next().unwrap_or(url).trim().to_owned();
    let status = metadata_lines.next().unwrap_or("0").trim();
    let content_type = metadata_lines
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_owned());
    }
    if !(content_type.starts_with("text/html")
        || content_type.starts_with("application/xhtml+xml")
        || content_type.starts_with("text/plain"))
    {
        return Err(format!(
            "tipo di contenuto non supportato ({})",
            if content_type.is_empty() {
                "sconosciuto"
            } else {
                &content_type
            }
        ));
    }

    let document = String::from_utf8_lossy(body);
    let (title, text, links) = if content_type.starts_with("text/plain") {
        (final_url.clone(), document.into_owned(), Vec::new())
    } else {
        parse_html(&document, &final_url)
    };

    Ok(Page {
        url: final_url,
        title: if title.is_empty() {
            format!("Pagina web ({status})")
        } else {
            title
        },
        text,
        links,
    })
}

fn parse_html(html: &str, base_url: &str) -> (String, String, Vec<Link>) {
    let title = extract_title(html);
    let mut text = String::new();
    let mut links = Vec::new();
    let mut active_link: Option<(String, String)> = None;
    let mut skip_tag: Option<String> = None;
    let mut cursor = 0;

    while cursor < html.len() {
        let Some(relative_start) = html[cursor..].find('<') else {
            if skip_tag.is_none() {
                append_text(&mut text, &html[cursor..]);
                if let Some((_, label)) = active_link.as_mut() {
                    append_text(label, &html[cursor..]);
                }
            }
            break;
        };
        let start = cursor + relative_start;
        if skip_tag.is_none() {
            let segment = &html[cursor..start];
            append_text(&mut text, segment);
            if let Some((_, label)) = active_link.as_mut() {
                append_text(label, segment);
            }
        }

        if html[start..].starts_with("<!--") {
            cursor = html[start + 4..]
                .find("-->")
                .map(|end| start + 4 + end + 3)
                .unwrap_or(html.len());
            continue;
        }

        let Some(end) = find_tag_end(html, start + 1) else {
            break;
        };
        let raw_tag = html[start + 1..end].trim();
        cursor = end + 1;
        if raw_tag.starts_with('!') || raw_tag.starts_with('?') || raw_tag.is_empty() {
            continue;
        }

        let closing = raw_tag.starts_with('/');
        let tag_body = raw_tag.strip_prefix('/').unwrap_or(raw_tag).trim_start();
        let name_end = tag_body
            .find(|character: char| character.is_whitespace() || character == '/')
            .unwrap_or(tag_body.len());
        let name = tag_body[..name_end].to_ascii_lowercase();

        if let Some(skipped) = skip_tag.as_ref() {
            if closing && &name == skipped {
                skip_tag = None;
            }
            continue;
        }

        if closing {
            if name == "a" {
                if let Some((href, label)) = active_link.take() {
                    if !label.trim().is_empty() && links.len() < MAX_LINKS {
                        if let Some(url) = resolve_url(base_url, &href) {
                            let number = links.len() + 1;
                            let label = clean_inline(&label);
                            if !text.is_empty() && !text.ends_with([' ', '\n']) {
                                text.push(' ');
                            }
                            text.push_str(&format!(" [{number}]"));
                            links.push(Link { label, url });
                        }
                    }
                }
            } else if is_block_tag(&name) {
                push_break(&mut text);
            }
            continue;
        }

        if matches!(
            name.as_str(),
            "script" | "style" | "noscript" | "svg" | "head"
        ) {
            skip_tag = Some(name);
            continue;
        }
        if name == "a" && active_link.is_none() {
            if let Some(href) = parse_attribute(tag_body, "href") {
                active_link = Some((href, String::new()));
            }
        }
        if name == "br" || name == "hr" || is_block_tag(&name) {
            push_break(&mut text);
        }
    }

    (title, normalize_text(&text), links)
}

fn extract_title(html: &str) -> String {
    let lower = html.to_ascii_lowercase();
    let Some(start_tag) = lower.find("<title") else {
        return String::new();
    };
    let Some(content_start_relative) = lower[start_tag..].find('>') else {
        return String::new();
    };
    let content_start = start_tag + content_start_relative + 1;
    let Some(end_relative) = lower[content_start..].find("</title") else {
        return String::new();
    };
    clean_inline(&html[content_start..content_start + end_relative])
}

fn find_tag_end(html: &str, mut index: usize) -> Option<usize> {
    let bytes = html.as_bytes();
    let mut quote = None;
    while index < bytes.len() {
        let byte = bytes[index];
        match (quote, byte) {
            (Some(current), value) if value == current => quote = None,
            (None, b'\'' | b'"') => quote = Some(byte),
            (None, b'>') => return Some(index),
            _ => {}
        }
        index += 1;
    }
    None
}

fn parse_attribute(tag_body: &str, requested: &str) -> Option<String> {
    let bytes = tag_body.as_bytes();
    let mut index = tag_body
        .find(|character: char| character.is_whitespace())
        .unwrap_or(tag_body.len());
    while index < bytes.len() {
        while index < bytes.len() && (bytes[index].is_ascii_whitespace() || bytes[index] == b'/') {
            index += 1;
        }
        let key_start = index;
        while index < bytes.len()
            && !bytes[index].is_ascii_whitespace()
            && !matches!(bytes[index], b'=' | b'/')
        {
            index += 1;
        }
        if index == key_start {
            break;
        }
        let key = tag_body[key_start..index].to_ascii_lowercase();
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        let mut value = String::new();
        if index < bytes.len() && bytes[index] == b'=' {
            index += 1;
            while index < bytes.len() && bytes[index].is_ascii_whitespace() {
                index += 1;
            }
            if index < bytes.len() && matches!(bytes[index], b'\'' | b'"') {
                let quote = bytes[index];
                index += 1;
                let value_start = index;
                while index < bytes.len() && bytes[index] != quote {
                    index += 1;
                }
                value = tag_body[value_start..index].to_owned();
                index = (index + 1).min(bytes.len());
            } else {
                let value_start = index;
                while index < bytes.len() && !bytes[index].is_ascii_whitespace() {
                    index += 1;
                }
                value = tag_body[value_start..index]
                    .trim_end_matches('/')
                    .to_owned();
            }
        }
        if key == requested {
            return Some(decode_entities(&value));
        }
    }
    None
}

fn append_text(output: &mut String, input: &str) {
    let decoded = decode_entities(input);
    for character in decoded.chars() {
        if character.is_whitespace() {
            if !output.is_empty() && !output.ends_with([' ', '\n']) {
                output.push(' ');
            }
        } else {
            output.push(character);
        }
    }
}

fn push_break(output: &mut String) {
    while output.ends_with(' ') {
        output.pop();
    }
    if !output.is_empty() && !output.ends_with('\n') {
        output.push('\n');
    }
}

fn normalize_text(input: &str) -> String {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn clean_inline(input: &str) -> String {
    let mut output = String::new();
    append_text(&mut output, input);
    output.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn decode_entities(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut cursor = 0;
    while cursor < input.len() {
        let rest = &input[cursor..];
        if rest.starts_with('&') {
            if let Some(end) = rest.find(';').filter(|end| *end <= 12) {
                let entity = &rest[1..end];
                let decoded = match entity {
                    "amp" => Some('&'),
                    "lt" => Some('<'),
                    "gt" => Some('>'),
                    "quot" => Some('"'),
                    "apos" | "#39" => Some('\''),
                    "nbsp" => Some(' '),
                    _ if entity.starts_with("#x") || entity.starts_with("#X") => {
                        u32::from_str_radix(&entity[2..], 16)
                            .ok()
                            .and_then(char::from_u32)
                    }
                    _ if entity.starts_with('#') => {
                        entity[1..].parse::<u32>().ok().and_then(char::from_u32)
                    }
                    _ => None,
                };
                if let Some(character) = decoded {
                    result.push(character);
                    cursor += end + 1;
                    continue;
                }
            }
        }
        let character = rest.chars().next().expect("cursor is within string");
        result.push(character);
        cursor += character.len_utf8();
    }
    result
}

fn is_block_tag(name: &str) -> bool {
    matches!(
        name,
        "address"
            | "article"
            | "aside"
            | "blockquote"
            | "body"
            | "dd"
            | "div"
            | "dl"
            | "dt"
            | "fieldset"
            | "figcaption"
            | "figure"
            | "footer"
            | "form"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "header"
            | "li"
            | "main"
            | "nav"
            | "ol"
            | "p"
            | "pre"
            | "section"
            | "table"
            | "td"
            | "th"
            | "tr"
            | "ul"
    )
}

fn resolve_url(base: &str, href: &str) -> Option<String> {
    let href = href.trim();
    if href.is_empty() || href.starts_with('#') {
        return None;
    }
    let lower = href.to_ascii_lowercase();
    if lower.starts_with("javascript:")
        || lower.starts_with("mailto:")
        || lower.starts_with("data:")
        || lower.starts_with("tel:")
    {
        return None;
    }
    if lower.starts_with("https://") || lower.starts_with("http://") {
        return Some(href.to_owned());
    }
    let (scheme, authority, base_path) = split_url(base)?;
    if href.starts_with("//") {
        return Some(format!("{scheme}:{href}"));
    }
    if href.starts_with('#') {
        return Some(format!("{base}{href}"));
    }
    if href.starts_with('?') {
        let path = base_path.split(['?', '#']).next().unwrap_or("/");
        return Some(format!("{scheme}://{authority}{path}{href}"));
    }
    let (href_path, suffix) = split_path_suffix(href);
    let joined = if href_path.starts_with('/') {
        href_path.to_owned()
    } else {
        let clean_base = base_path.split(['?', '#']).next().unwrap_or("/");
        let directory = clean_base
            .rsplit_once('/')
            .map(|(dir, _)| dir)
            .unwrap_or("");
        format!("{directory}/{href_path}")
    };
    Some(format!(
        "{scheme}://{authority}{}{suffix}",
        normalize_path(&joined)
    ))
}

fn split_url(url: &str) -> Option<(&str, &str, &str)> {
    let (scheme, remainder) = url.split_once("://")?;
    let authority_end = remainder.find(['/', '?', '#']).unwrap_or(remainder.len());
    let authority = &remainder[..authority_end];
    if authority.is_empty() {
        return None;
    }
    let path = if authority_end == remainder.len() {
        "/"
    } else if remainder.as_bytes()[authority_end] == b'/' {
        &remainder[authority_end..]
    } else {
        "/"
    };
    Some((scheme, authority, path))
}

fn split_path_suffix(path: &str) -> (&str, &str) {
    let suffix_start = path.find(['?', '#']).unwrap_or(path.len());
    (&path[..suffix_start], &path[suffix_start..])
}

fn normalize_path(path: &str) -> String {
    let mut segments = Vec::new();
    for segment in path.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                segments.pop();
            }
            _ => segments.push(segment),
        }
    }
    format!("/{}", segments.join("/"))
}
