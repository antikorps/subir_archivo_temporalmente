use reqwest;
use tokio;
use clap::Parser;

mod modelos;
mod manejador;
mod utilidades;
mod gui;

/// Utilidad para subir archivos temporalmente a internet a través de https://0x0.st/
#[derive(Parser, Debug)]
struct Argumentos {
    /// tiempo en horas que el archivo debe estar disponible, por defecto 168 (7 días)
    #[arg(short, long, default_value_t = 168)]
    tiempo: u8,

    /// evitar crear el archivo de resumen en json
    #[arg(short, long, default_value_t = false)]
    nojson: bool,
}

#[tokio::main]
async fn main() {
    let argumentos = Argumentos::parse();
    let mut manejador = modelos::Manejador{
        tiempo: argumentos.tiempo,
        nojson: argumentos.nojson,
        archivos: utilidades::seleccionar_archivos(),
        cliente: reqwest::Client::new(),
        json: "".to_string(),
    };
    let gui = manejador.procesar_archivos().await;
   
    gui::mostrar_gui(gui);
}