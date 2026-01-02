[![Built With Ratatui](https://img.shields.io/badge/Built_With_Ratatui-000?logo=ratatui&logoColor=fff)](https://ratatui.rs/)
[![Static Badge](https://img.shields.io/badge/MIT-blue?style=flat&label=license&labelColor=gray)](LICENSE)


# Not The End – Local TUI Game Helper

Un helper locale basato su terminale per giocare a **Not the End**, il TTRPG narrativo di Claudio Pustorino, quando non è possibile giocare dal vivo.

Il progetto cerca di preservare il più possibile il flusso narrativo e la fisicità del gioco, offrendo un’interfaccia leggera, locale e priva di distrazioni.


## Cos’è Not the End

[Not the End](https://drive.google.com/file/d/15TJxFvZwyrG9Cei4qxI9_EaWFoh74O-J/view) è un gioco di ruolo da tavolo fortemente orientato alla narrazione, costruito attorno a un’idea semplice ma geniale che dà il meglio di sé nel gioco in presenza.

Quando si gioca a distanza, però, parte di questa magia tende a perdersi, anche a causa della scarsità di [strumenti online dedicati](https://docs.google.com/presentation/d/1snKsZr42zJAx2Y3Ft_80-0t8Jbhvkzt14vUUpnQCSCw/copy).

Questo progetto nasce per colmare proprio questo vuoto.


## Funzionalità

- Interfaccia TUI (Terminal User Interface) basata su [Ratatui](https://ratatui.rs/)
- Gestione delle sfide di *Not the End*
- Foglio personaggio ispirato all’HexSys
- Dati del personaggio persistenti in formato TOML
- Navigazione tramite tastiera e mouse
- Storico delle sfide della sessione corrente
- Completamente locale: nessun account, nessun server, nessuna connessione richiesta


## Screenshot & Video

### Risoluzione delle Sfide
[![asciicast](https://asciinema.org/a/6nScyDSCPyEqfHAi6vWl7HO7S.svg)](https://asciinema.org/a/6nScyDSCPyEqfHAi6vWl7HO7S)

### Foglio Personaggio (Schede 2 & 3)
[![asciicast](https://asciinema.org/a/6rCh124LhcXLi4sPQ1Hnxil18.svg)](https://asciinema.org/a/6rCh124LhcXLi4sPQ1Hnxil18)

### Storico delle Sfide
[![asciicast](https://asciinema.org/a/3cyCQHyw8SvVhDUmortktd0FY.svg)](https://asciinema.org/a/3cyCQHyw8SvVhDUmortktd0FY)


## Come Funziona

L’applicazione salva un insieme minimo di informazioni del foglio personaggio in un file `character_sheet.toml`, posizionato nella stessa directory dell’eseguibile.

Se il file non è presente, viene creato automaticamente al primo avvio.

A causa dei limiti del terminale, la struttura a nido d’ape dell’HexSys non può essere riprodotta fedelmente e viene quindi approssimata tramite celle rettangolari.


## Utilizzo

L’interfaccia è suddivisa in quattro schede:

1. **Sfide**  
   Aggiunta dei token dei tratti, token di difficoltà del narratore, modificatori di stato, pesca e rischio.

2. **Foglio Personaggio (Parte I)**  
   La parte principale del foglio ispirato all’HexSys.

3. **Foglio Personaggio (Parte II)**  
   Informazioni aggiuntive e note sul personaggio.

4. **Storico delle Sfide**  
   Registro temporaneo delle sfide affrontate durante la sessione.

⚠️ Lo storico delle sfide **non viene salvato** alla chiusura dell’applicazione.


## Comandi da Tastiera

| Tasto          | Azione                                |
|----------------|---------------------------------------|
| Frecce         | Navigazione tra gli elementi          |
| Mouse Sinistro | Selezione elementi UI                 |
| Enter          | Conferma / Modifica campo selezionato |
| Esc            | Annulla / Conferma modifica           |
| E              | Abilita campo selezionato             |
| R              | Reset della sfida corrente            |
| Q              | Uscita dall'applicazione              |


## Installazione

Compila il progetto a partire dal codice sorgente:

```bash
git clone https://github.com/DrStiev/Not_The_End.git
cd Not_The_End
cargo build --release
cp target/release/Not_The_End <PERCORSO_DESIDERATO>
```

## Stato del Progetto

Il progetto è completo per uso personale, ma aperto a miglioramenti ed estensioni.

Feedback, suggerimenti e contributi sono benvenuti.


## Crediti 

- Not the End — Claudio Pustorino

- Ratatui — framework TUI per Rust


## Licenza

Questo progetto è distribuito sotto [licenza MIT](LICENSE).