# Changelog

## 0.2.8

- Aggiunti il pacchetto RPM per Fedora e openSUSE Tumbleweed e la PKGBUILD per Arch Linux e Manjaro.
- Aggiunta una pipeline GitHub Actions che costruisce i pacchetti nativi e li allega alle release.
- Aggiornata la documentazione per installazione, dipendenze e stato della licenza del progetto.

## 0.2.7

- Aggiunti i temi chiaro e scuro per l'interfaccia, con tema GTK di sistema predefinito.
- Aggiunto un selettore compatto nella barra superiore; le scelte manuali durano per la sessione e non vengono salvate.

## 0.2.6

- Attivato un profilo WebKitGTK temporaneo: cookie, accessi, cache e dati dei siti non vengono conservati dopo la chiusura.
- Annullati i download per evitare che il browser salvi file sul computer.
- Aggiornata la documentazione chiarendo i limiti rispetto ai dati già conservati dai siti e alle versioni precedenti.

## 0.2.5

- Aggiunta la gestione della pressione memoria WebKitGTK con budget di 512 MB per processo web e processo di rete, chiedendo al motore di liberare memoria non essenziale quando serve.
- Conservati i dati web esistenti, inclusi cookie e login, usando le stesse cartelle profilo.
- Aggiornati requisiti e documentazione sui limiti della soglia: è adattiva e non garantisce un tetto massimo assoluto.

## 0.2.4

- Corretto il problema per cui alcuni video, incluso YouTube, potevano non essere visualizzati dopo aver disattivato del tutto l'accelerazione hardware.
- Impostata l'accelerazione hardware su richiesta del sito; restano disattivate cache delle risorse, cache back-forward e WebGL per contenere l'uso di memoria.
- Aggiornata la documentazione per spiegare il compromesso tra memoria, compatibilità video e prestazioni.

## 0.2.3

- Risolto il problema dell'uso eccessivo di RAM attribuito a WebKitGTK riducendo al minimo le cache, disattivando l'accelerazione hardware e mantenendo disattivati WebGL e la cache delle pagine.
- Aggiornata la documentazione per spiegare le modifiche e i compromessi: maggiore uso possibile della CPU, ricaricamenti più lenti e nessun limite fisso alla memoria usata dalle pagine o dai video.

## 0.2.2

- Aggiunta la nuova icona vettoriale del progetto e collegata al menu applicazioni e alla finestra.
- Documentazione ampliata per installazione, requisiti e crediti.

## 0.2.1

- Interfaccia più compatta e impostazioni WebKitGTK orientate a un uso più contenuto della memoria.
- JavaScript resta attivo; WebGL e cache delle pagine sono disattivati.

## 0.2.0

- Sostituito il parser HTML testuale della modalità grafica con WebKitGTK.
- Aggiunto supporto a JavaScript, CSS, cookie e pagine interattive.

## 0.1.0

- Prima versione testuale, con URL, ricerca, cronologia e link numerati.
