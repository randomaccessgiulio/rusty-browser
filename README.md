# old bad looking browser (a.k.a. A Rusty Browser)

Browser desktop minimale scritto in Rust, con **WebKitGTK** come motore per visualizzare le pagine. 

![Icona Rusty Browser](assets/rusty-browser.png)

## 1. Cos'è

A Rusty Browser è una piccola applicazione per Linux con una barra degli indirizzi compatta e le funzioni essenziali di navigazione. WebKitGTK gestisce HTML, CSS, JavaScript, cookie e login solo per la sessione aperta. Alla chiusura, i dati dei siti vengono scartati. È un progetto personale e in evoluzione, non un sostituto completo di Firefox o Chromium.

## 2. A chi si rivolge

- A chi vuole una finestra browser essenziale, con i comandi principali sempre visibili.
- A chi usa Ubuntu e preferisce un'applicazione Linux integrata con il motore WebKitGTK di sistema.
- A chi vuole esplorare o contribuire a un piccolo progetto Rust con interfaccia GTK.

## 3. Perché provarlo

- Interfaccia ridotta a indirizzo, vai, indietro, avanti e ricarica.
- Temi dell'interfaccia chiaro e scuro, con **Sistema** selezionato all'avvio.
- Motore WebKitGTK aggiornato tramite gli aggiornamenti del sistema.
- JavaScript attivo; accessi e cookie funzionano durante la sessione e vengono scartati alla chiusura.
- Navigazione temporanea: i dati dei siti non vengono conservati tra un avvio e l'altro.
- Impostazioni WebKitGTK orientate a contenere l'uso di memoria: budget di pressione memoria da 512 MB per processo web, cache delle pagine e WebGL disattivati; accelerazione hardware usata solo quando richiesta dal sito.
- Codice Rust e pacchetto Debian riproducibile con lo script incluso.


## Tema dell'interfaccia

Il selettore **Sistema / Chiaro / Scuro** è nella barra superiore. **Sistema** segue il tema GTK attivo sul desktop; le scelte manuali cambiano solo l'interfaccia del browser, non i colori dei siti. La scelta dura per la sessione corrente e non viene salvata.

## Privacy e dati locali

Ogni avvio crea una sessione WebKit temporanea. Cookie, accessi, archiviazione dei siti e cache restano disponibili mentre il browser è aperto, poi vengono scartati. Rusty Browser non registra una cronologia di navigazione e annulla i download invece di salvare file. Per questo dovrai accedere di nuovo ai siti a ogni avvio.

Questa impostazione riguarda i dati locali di Rusty Browser: non può cancellare l'attività già registrata dai siti web, dai servizi in cui accedi, dalla rete o dal provider internet. I dati persistenti lasciati da versioni precedenti di WebKitGTK non vengono rimossi automaticamente.

## 4. Requisiti e risorse

### Per installare e usare il pacchetto

- Ubuntu **26.10**, architettura **amd64** (Intel/AMD a 64 bit).
- Una sessione grafica Linux, X11 o Wayland.
- `libwebkit2gtk-4.1-0` versione **2.34 o successiva**, che porta con sé GTK 3 e le altre librerie runtime richieste.
- `curl`, usato solo dalla modalità testuale.

Il pacchetto Debian dichiara le dipendenze runtime; `apt` le risolve automaticamente.

### Per compilare dai sorgenti

- `rustc` e `cargo`.
- `dpkg-deb` per generare il pacchetto Debian.
- Le librerie runtime WebKitGTK/GTK sopra indicate.
- Nessuna crate Rust esterna: il progetto collega le librerie WebKitGTK e GTK presenti nel sistema.

Su Ubuntu, se gli strumenti non sono già installati:

```sh
sudo apt update
sudo apt install rustc cargo dpkg libwebkit2gtk-4.1-0 curl
```

## Installazione

Scarica `rusty-browser_0.2.7_amd64.deb` dalla cartella `dist/` del repository o dalla [pagina Releases](https://github.com/randomaccessgiulio/rusty-browser/releases/latest), poi esegui dalla cartella in cui hai salvato il file:

```sh
sudo apt install ./rusty-browser_0.2.7_amd64.deb
```

L'installazione aggiunge Rusty Browser al menu applicazioni. Per rimuoverlo:

```sh
sudo apt remove rusty-browser
```

## Avvio e uso

Avvia ** A Rusty Browser** dal menu applicazioni. Scrivi un indirizzo completo, oppure una ricerca, nella barra superiore e premi Invio o **Vai**. Usa le frecce per la cronologia e il simbolo di ricarica per aggiornare la pagina; passa il puntatore sui pulsanti per leggerne le etichette.

La modalità grafica è quella predefinita:

```sh
cargo run --release
```

La modalità testuale è disponibile con `--text`; mostra testo e link ma non esegue JavaScript:

```sh
cargo run --release -- --text
```

## Creare il pacchetto Debian

Da questa cartella esegui:

```sh
./package-deb.sh
```

Lo script compila la versione release e crea il `.deb` sia nella cartella `outputs` superiore sia in `dist/`.

## Contenuto del repository

- `src/` — applicazione Rust e interfaccia WebKitGTK.
- `assets/rusty-browser.svg` e `assets/rusty-browser.png` — icona vettoriale e anteprima raster.
- `packaging/` — voce del menu desktop.
- `package-deb.sh` — script per generare il pacchetto Debian.
- `CHANGELOG.md` — cronologia delle versioni.
- `CREDITS.md` — crediti del progetto.

## Licenza

La licenza non è ancora stata definita dall'autore.

## Crediti

Vedi [CREDITS.md](CREDITS.md): il progetto è ideato da **Giulio (@itsgiuliooo)** con il supporto di **ChatGPT**
