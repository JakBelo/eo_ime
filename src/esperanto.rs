use std::{collections::VecDeque, sync::Mutex};

static ELIGA_VICO: Mutex<VecDeque<char>> = Mutex::new(VecDeque::new());

pub fn anstatauxigi(c: char, stato: &mut String) -> Option<char> {
    // 1. Unue kontroli ĉu estas signoj en la eliga vico por elsendi.
    if let Ok(mut vico) = ELIGA_VICO.lock() {
        if let Some(vicigita_signo) = vico.pop_front() {
            // Trakti la nunan enigon: se la eliga vico enhavas atendajn signojn,
            // La nuna enigo ‘c’ devas esti ĝuste traktata.
            if stato.is_empty() {
                stato.push(c);
            } else {
                // Se stato ne estas malplena, tio signifas ke estas netraktitaj signoj.
                // Enmeti ‘c’ en la eligan vicon.
                vico.push_back(c);
            }
            return Some(vicigita_signo);
        }
    }
    
    // 2. Normale trakti la enigon.
    stato.push(c);
    
    match stato.as_str() {
        "cx" => { stato.clear(); Some('ĉ') }
        "gx" => { stato.clear(); Some('ĝ') }
        "hx" => { stato.clear(); Some('ĥ') }
        "jx" => { stato.clear(); Some('ĵ') }
        "sx" => { stato.clear(); Some('ŝ') }
        "ux" => { stato.clear(); Some('ŭ') }
        "Cx" => { stato.clear(); Some('Ĉ') }
        "Gx" => { stato.clear(); Some('Ĝ') }
        "Hx" => { stato.clear(); Some('Ĥ') }
        "Jx" => { stato.clear(); Some('Ĵ') }
        "Sx" => { stato.clear(); Some('Ŝ') }
        "Ux" => { stato.clear(); Some('Ŭ') }

        "c" | "g" | "h" | "j" | "s" | "u" 
        | "C" | "G" | "H" | "J" | "S" | "U" => None,

        _ => {
            // Ne estas speciala sinsekvo, do elsendu ĉiujn signojn.
            let mut signoj: Vec<char> = stato.drain(..).collect();
            let unua = signoj.remove(0);  // La unua signo estu tuj redonita.
            
            // Enmeti la ceterajn signojn en la eligan vicon.
            if !signoj.is_empty() {
                if let Ok(mut vico) = ELIGA_VICO.lock() {
                    for ch in signoj {
                        vico.push_back(ch);
                    }
                }
            }
            
            Some(unua)
        }
    }
}
