mod app;
mod input;
mod terminal;
mod ui;

/// Entry point dell'applicazione
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inizializza il terminale
    terminal::setup()?;

    // Crea l'applicazione e avvia il loop principale
    let mut app = app::App::new();
    // Cleanup è gestito nel match del risultato
    run_app(&mut app)
}

/// Loop principale dell'applicazione
fn run_app(app: &mut app::App) -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal| {
        loop {
            // Renderizza l'UI
            let _ = terminal.draw(|frame| ui::ui(frame, app));

            // Gestisci input
            match input::handle_input(app) {
                Ok(should_quit) => {
                    if should_quit {
                        // Cleanup e termina
                        terminal::cleanup(terminal)?;
                        break Ok(());
                    }
                }
                Err(e) => {
                    // In caso di errore, fai comunque cleanup
                    let _ = terminal::cleanup(terminal);
                    break Err(Box::new(e) as Box<dyn std::error::Error>);
                }
            }
        }
    })
}

// #[cfg(test)]
// mod tests {

//     use crate::app::App;
//     use crate::ui::ui;
//     use insta::assert_snapshot;
//     use ratatui::{Terminal, backend::TestBackend};

//     #[test]
//     fn test_render_app() {
//         let mut app = App::new();
//         let mut terminal = Terminal::new(TestBackend::new(100, 40)).unwrap();
//         terminal.draw(|frame| ui(frame, &mut app)).unwrap();
//         assert_snapshot!(terminal.backend());
//     }
// }
