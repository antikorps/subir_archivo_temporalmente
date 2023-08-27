use web_view::*;
use crate::modelos;

pub fn componer_gui(resultados: Vec<modelos::Resultado>) -> String {
    let mut gui = r#"
    <!DOCTYPE html>
    <html lang="es">
    
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Resultados</title>
        <style>
            body {
                padding: 20px;
                background: #EBF0F5;
            }
            p {
                color: #333;
                font-family: "Helvetica Neue", sans-serif;
                font-size: 20px;
                margin: 0;
                line-height: 200%;
            }
            .tarjeta {
                background: white;
                padding: 60px;
                border-radius: 4px;
                max-width: 80%;
                margin: 0 auto 30px auto;
            }
            .exito {
                box-shadow: 3px 2px 3px #9ABC66;
            }
            .error {
                box-shadow: 3px 2px 3px tomato;
            }
        </style>
    </head>
    
    <body>
"#.to_string();

    for resultado in resultados {
        if resultado.error != "" {
            let tarjeta = format!("<div class=\"tarjeta error\">
<p>{}</p>
</div>", resultado.error);
            gui += &tarjeta;
        } else {
            let tarjeta = format!("<div class='tarjeta exito'>
<p><strong>Archivo</strong>: {}</p>
<p><strong>URL</strong>: {}</p>
<p><strong>Token</strong>:{}. Eliminar con: {}</p>
<p><strong>Disponible hasta</strong>: {} (unixtime: {})</p>
</div>", 
            resultado.info.ruta,
            resultado.info.url,
            resultado.info.token,
            resultado.info.eliminar,
            resultado.info.disponibilidad,
            resultado.info.expiracion);
            gui += &tarjeta;
        }
    }

    gui += "</body></html>";

    return gui;
}

pub fn mostrar_gui(gui: String) {
    web_view::builder()
        .title("Resultados")
        .content(Content::Html(gui))
        .size(800, 600)
        .resizable(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}

