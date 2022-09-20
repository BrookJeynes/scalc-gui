use serde::{ Serialize, Deserialize };

#[cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[derive(Serialize, Deserialize)]
struct Output {
    subnet: i32,
    networkAddress: String,
    subnetMask: String,
    usableIpRange: String,
    broadcastAddress: String
}

#[tauri::command]
fn display_ip(ip_address: String, slash_notation: i8) -> Vec<String> {
    // Output table rows
    let mut rows = Vec::new();
    
    let mut host_id_bits: i8 = 8;
    let mut borrowed_bits: i32 = 1_f64.log2().ceil() as i32;
    let mut ip_address_per_subnet: i32 = i32::pow(2, host_id_bits as u32 - borrowed_bits as u32); 

    if slash_notation > 24 {
        host_id_bits = 8 - (slash_notation - 24);
        borrowed_bits = (8 - host_id_bits).into();
        ip_address_per_subnet = i32::pow(2, host_id_bits as u32);
    }

    let subnets_needed: i32 = i32::pow(borrowed_bits, 2);
   
    let loop_limit: i32 = 256 + ip_address_per_subnet;

    // Format IP address
    let split = ip_address.split(".");
    let mut ip_address_mod = split.collect::<Vec<&str>>();
    ip_address_mod.pop();

    let new_ip_address_mod = ip_address_mod.join(".");

    for i in 0..loop_limit {
        if i % ip_address_per_subnet == 0 && i - ip_address_per_subnet >= 0 { 
            let instantiated: Output = Output {
                subnet: (rows.len() + 1) as i32,
                networkAddress: String::from(format!("{}.{}", new_ip_address_mod, i-ip_address_per_subnet)),
                subnetMask: String::from(format!("255.255.255.255/{}", slash_notation)),
                usableIpRange: String::from(format!("{}.{} - {}.{}", new_ip_address_mod, ((i-ip_address_per_subnet)+1), new_ip_address_mod, i-2)),
                broadcastAddress: String::from(format!("{}.{}", new_ip_address_mod, i-1))
            };

            let json = serde_json::to_string(&instantiated).unwrap();

            rows.push(json);
        }
    }

    rows.into()
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![display_ip])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
