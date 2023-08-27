use std::fs;
use reqwest;
use futures::future::try_join_all;

use crate::utilidades;
use crate::modelos;
use crate::gui;

async fn subir_archivo(cliente: &reqwest::Client, ruta: String, disponibilidad: String) -> Result<modelos::InfoSubida,String> {
    let archivo;
    match fs::read(ruta.clone()) {
        Err(error) => {
            let mensaje_error = format!("ERROR: no se ha podido leer el archivo: {ruta}: {error}", );
            return Err(mensaje_error);
        }
        Ok(ok) => {
            archivo = ok
        }
    }
    let parte = reqwest::multipart::Part::bytes(archivo).file_name("");
    let parte_expires = reqwest::multipart::Part::text(disponibilidad);
    let form = reqwest::multipart::Form::new().part("file", parte).part("expires", parte_expires);
    let respuesta;
    match cliente.post("https://0x0.st").multipart(form).send().await {
        Err(error) => {
            let mensaje_error = format!("ERROR: no se ha enviar la petición para subir leer el archivo: {ruta}: {error}");
            return Err(mensaje_error);
        }
        Ok(ok) => {
            respuesta = ok
        }
    }
    if respuesta.status().as_u16() != 200 {
        let mensaje_error = format!("ERROR: la petición para subir el archivo {ruta} ha devuelto un status code incorrecto {}", respuesta.status());
        return Err(mensaje_error);
    }
    let cabeceras = respuesta.headers();
    let mut token;
    match cabeceras.get("X-Token") {
        None => {
            token = "".to_string()
        }
        Some(ok) => {
            let valor = ok.to_str().unwrap_or("");
            token = valor.to_string();
        }
    }
    let mut expiracion = "".to_string();
    match cabeceras.get("X-expires") {
        None => {
            token = "".to_string()
        }
        Some(ok) => {
            let valor = ok.to_str().unwrap_or("");
            expiracion = valor.to_string();
        }
    }
    let mut disponibilidad = "".to_string();
    if expiracion != "" {
        match utilidades::legibilizar_cadena_unixtime(&expiracion) {
            Err(error) => {
                eprintln!("{error}")
            }
            Ok(fecha) => {
                disponibilidad = fecha;
            }
        }
    }
    
    match respuesta.text().await {
        Err(error) => {
            let mensaje_error = format!("ERROR: no se ha obtener el texto de la respuesta subir leer el archivo: {ruta}: {error}");
            return Err(mensaje_error);
        }
        Ok(ok) => {
            let url_definitiva = ok.replace("\n", "");
            let eliminar = format!("curl -Ftoken={token} -Fdelete= {url_definitiva}");
            return Ok(modelos::InfoSubida { ruta, url: url_definitiva, token, eliminar, expiracion, disponibilidad });
        }
    }
}

impl modelos::Manejador {
    pub async fn procesar_archivos(&mut self) -> String{
        
        let mut futuros = Vec::new();

        for ruta in self.archivos.clone() {
            futuros.push(subir_archivo(&self.cliente, ruta, format!("{}", self.tiempo)))
        }
        let mut resultados = Vec::new();
        match try_join_all(futuros).await {
            Err(error) => {
                let resultado = modelos::Resultado {
                    error: error,
                    info: modelos::InfoSubida { 
                        ruta: "".to_string(),
                        url: "".to_string(), 
                        token: "".to_string(),
                        eliminar: "".to_string(),
                        expiracion: "".to_string(),
                        disponibilidad: "".to_string(),
                    }

                };
                resultados.push(resultado);
            }
            Ok(respuestas) => {
                for resp in respuestas {
                    let resultado = modelos::Resultado {
                        error: "".to_string(),
                        info: resp,
                    };
                    resultados.push(resultado);
                }
            }
        }

        if !self.nojson {
            match utilidades::persistir_json(&resultados) {
                Err(error) => {
                    eprintln!("{error}")
                }
                Ok(_) => ()
            }
        }

        let gui = gui::componer_gui(resultados);

        return gui
    }
}