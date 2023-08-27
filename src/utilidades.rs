use chrono::{Local, TimeZone};
use std::process;
use chrono::Utc;
use std::fs;
use std::io::Write; 

use crate::modelos;
pub fn seleccionar_archivos() -> Vec<String>{
    match tinyfiledialogs::open_file_dialog_multi("Selecciona los archivos", "", Some((&["*.*"], "Todos los archivos"))) {
        None => {
            process::exit(0)
        }
        Some(ok) => {
            return ok;
        }
    }
}

pub fn legibilizar_cadena_unixtime(tiempo: &str) -> Result<String, String> {
    let unixtime;
    match tiempo.parse::<i64>() {
        Err(error) => {
            let mensaje_error = format!("ERROR: no se ha podido parsear a i64 el tiempo recuperado: {tiempo}: {}", error.to_string());
            return Err(mensaje_error);
        }
        Ok(ok) => {
            unixtime = ok;
        }
    }
    let unixtime_segundos = unixtime / 1000;

    let fecha_local = Local.timestamp_opt(unixtime_segundos, 0);

    match fecha_local.single() {
        None => {
            let mensaje_error = format!("ERROR: no se ha podido obtener la fecha local del tiempo recuperado: {tiempo}");
            return Err(mensaje_error);
        }
        Some(single) => {
            Ok(single.format("%Y-%m-%d %H:%M:%S").to_string())
        }
    }
}

pub fn persistir_json(resultados: &Vec<modelos::Resultado>) -> Result<String,String> {
    let json;
    match serde_json::to_string(&resultados) {
        Err(error) => {
            let mensaje_error = format!("no se ha podido obtener el json de los resultados: {error}");
            return Err(mensaje_error);
        }
        Ok(ok) => {
            json = ok
        }
    }
    let ahora = Utc::now();
    let nombre_archivo = format!("{}.json", ahora.timestamp());

    let mut archivo_json;
    match fs::File::create(nombre_archivo) {
        Err(error) => {
            let mensaje_error = format!("no se ha podido crear el archivo json con los resultados: {error}");
            return Err(mensaje_error);
        }
        Ok(ok) => {
            archivo_json = ok
        }
    } 
    match archivo_json.write_all(json.as_bytes()) {
        Err(error) => {
            let mensaje_error = format!("error escribiendo el archivo json con los resultados: {error}");
            return Err(mensaje_error);
        }
        Ok(_) => {
            return Ok("ok".to_string());
        }
    }
}