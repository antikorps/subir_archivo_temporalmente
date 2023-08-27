use serde::Serialize;
pub struct Manejador {
    pub archivos: Vec<String>,
    pub tiempo: u8,
    pub nojson: bool,
    pub cliente: reqwest::Client,
    pub json: String,
}
#[derive(Serialize)]
pub struct InfoSubida {
    pub ruta: String,
    pub url: String,
    pub token: String,
    pub eliminar: String,
    pub expiracion: String,
    pub disponibilidad: String,
}
#[derive(Serialize)]
pub struct Resultado {
    pub error: String,
    pub info: InfoSubida,
}