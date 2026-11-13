# Changelog

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
