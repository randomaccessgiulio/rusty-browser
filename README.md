# old bad looking browser (a.k.a. A Rusty Browser)

Browser desktop minimale scritto in Rust, con **WebKitGTK** come motore per visualizzare le pagine. Versione corrente: **0.2.5**.

![Icona Rusty Browser](assets/rusty-browser.png)

## 1. Cos'è

A Rusty Browser è una piccola applicazione per Linux con una barra degli indirizzi compatta e le funzioni essenziali di navigazione. WebKitGTK gestisce HTML, CSS, JavaScript, cookie e login, così le pagine interattive possono funzionare come in un browser moderno. È un progetto personale e in evoluzione, non un sostituto completo di Firefox o Chromium.

## 2. A chi si rivolge

- A chi vuole una finestra browser essenziale, con i comandi principali sempre visibili.
- A chi usa Ubuntu e preferisce un'applicazione Linux integrata con il motore WebKitGTK di sistema.
- A chi vuole esplorare o contribuire a un piccolo progetto Rust con interfaccia GTK.

## 3. Perché provarlo

- Interfaccia ridotta a indirizzo, vai, indietro, avanti e ricarica.
- Motore WebKitGTK aggiornato tramite gli aggiornamenti del sistema.
- JavaScript e cookie attivi per accedere a siti interattivi e account.
- Impostazioni WebKitGTK orientate a contenere l'uso di memoria: budget di pressione memoria da 512 MB per processo web, cache delle pagine e WebGL disattivati; accelerazione hardware usata solo quando richiesta dal sito.
- Codice Rust e pacchetto Debian riproducibile con lo script incluso.

La release 0.2.3 aveva disattivato completamente l'accelerazione hardware per ridurre la RAM, ma questo poteva impedire la visualizzazione di alcuni video. Dalla 0.2.4 WebKitGTK può attivarla quando la pagina ne ha bisogno. La 0.2.5 aggiunge una soglia di pressione memoria da 512 MB per ciascun processo web: superate le soglie interne, WebKit prova a liberare memoria non essenziale e dati in cache. Il processo di rete riceve la stessa impostazione. È una richiesta di recupero della memoria, non un tetto rigido: pagine complesse e video, incluso YouTube, possono ancora superare la soglia, ma WebKit dovrebbe contenere meglio i picchi. La pagina non viene chiusa automaticamente. Se il sistema è molto sotto pressione, la pulizia può rendere temporaneamente meno fluida la pagina. Cache delle pagine e WebGL restano disattivati; i siti 3D basati su WebGL hanno funzioni limitate.

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

Scarica `rusty-browser_0.2.5_amd64.deb` dalla cartella `dist/` del repository o dalla [pagina Releases](https://github.com/randomaccessgiulio/rusty-browser/releases/latest), poi esegui dalla cartella in cui hai salvato il file:

```sh
sudo apt install ./rusty-browser_0.2.5_amd64.deb
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

Vedi [CREDITS.md](CREDITS.md): il progetto è ideato da **Giulio (@itsgiuliooo)** con il supporto di **ChatGPT** per progettazione, sviluppo, icona e documentazione.
