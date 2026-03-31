use serde::{Serialize, Deserialize};
use std::process::Command;
use std::env;

#[derive(Serialize, Deserialize)]
struct WifiNetwork {
    ssid: String,
    signal: String,
    security: String,
}

#[derive(Serialize, Deserialize)]
struct WifiStatus {
    ssid: String,
    signal: i32,
    icon: String,
    percentage: String,
}

//------------------------------------------------
//   FUNCION QUE HACE LAS LISTAS DE REDES        |
//------------------------------------------------
fn get_wifi_list() -> String {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "SSID,SIGNAL,SECURITY", "device", "wifi", "list"])
        .output()
        .expect("Fallo al ejecutar nmcli");
    let raw = String::from_utf8_lossy(&output.stdout);
    let mut networks = Vec::new();

    for line in raw.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 3 && !parts[0].is_empty(){
            networks.push(WifiNetwork{
                ssid: parts[0].to_string(),
                signal:parts[1].to_string(),
                security: parts[2].to_string(),
            });
        }
    }
    serde_json::to_string(&networks).unwrap()
}

//-------------------------------------------------------
//      FUNCION PARA VER EL STADO DE LA RED             |
//-------------------------------------------------------
fn get_wifi_status() -> String {
    let output = Command::new("nmcli")
        //FURZA AL IDIOMA EN INGLES
        .env("LC_ALL", "C")
        .args(["-t", "-f", "ACTIVE.+,SSID,SIGNAL", "dev", "wifi"])
        .output()
        .expect("fallo al ejecutar nmcli");
    let raw = String::from_utf8_lossy(&output.stdout);

    for line in raw.lines() {
        if line.starts_with("yes"){
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                let ssid = parts[1].to_string();
                let signal: i32 = parts[2].parse().unwrap_or(0);
                //logica de iconos
                let icon = if signal >=75 { "󰒢" }
                            else if signal >= 50 { "󰢽" }
                            else if signal >= 25 { "󰢼" }
                            else { "󰢿" };

                let status = WifiStatus {
                    ssid,
                    signal,
                    icon: icon.to_string(),
                    percentage: format!("{}%", signal),
                };
                return serde_json::to_string(&status).unwrap();
            }
        }
    }
//si no se encuentra nada activo, devuelve el estado a desconectado
    serde_json::to_string(&WifiStatus {
        ssid: "Desconectado".to_string(),
        signal: 0, 
        icon: "󰢿".to_string(),
        percentage: "0%".to_string(),
    }).unwrap()
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Uso: Hyprconf-backend [list|status|connect]");
        return;
    }
    match args[1].as_str() {
        "list" => println!("{}", get_wifi_list()),
        "status" => println!("{}", get_wifi_status()),
        _ => println!("Comando no reconocido"),
    }
}
