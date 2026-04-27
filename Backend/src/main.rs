use std::process::Command;
use serde_json::json;

//------------------------------------------------
//   FUNCION QUE HACE LAS LISTAS DE REDES        |
//------------------------------------------------

fn get_wifi_statu() -> serde_json::Value {
    let output = Command::new("/usr/bin/nmcli")
        .env("LC_ALL", "C")
        .args(&["-t", "-f", "ACTIVE,SSID,SIGNAL", "dev", "wifi"])
        .output()
        .expect("Fallo al ejecutar nmcli");

    let stdout = String::from_utf8_lossy(&output.stdout); 

    for line in stdout.lines() {

        if line.starts_with("yes") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >=3 {
               return json!({
                    "connected" : true,
                    "ssid": parts[1],
                    "signal": parts[2].parse::<i32>().unwrap_or(0),
                }); 
            }    
        }
    }
    json!({
        "connected": false, 
        "ssid": "Desconectado", 
        "signal": 0
    })
}

//------------------------------------------------
//          FUNCION PRINCIPAL LA MAIN            |
//------------------------------------------------

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "--wifi-status" {
        println!("{}", get_wifi_statu());
    } else {
        eprint!("Uso: hyperconf --wifi-status");
    }
}
