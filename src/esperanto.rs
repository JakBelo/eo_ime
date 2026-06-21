pub fn anstatauxigi(c: char, stato: &mut String) -> Option<char> {
    if stato.is_empty() {
        // La stato estas malplena.
        if matches!(c, 'c' | 'g' | 'h' | 'j' | 's' | 'u' |
                        'C' | 'G' | 'H' | 'J' | 'S' | 'U') {
            stato.push(c);
            None
        } else {
            Some(c)
        }
    } else {
        // En la stato estas unu signo.
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
            
            _ => {
                // Ne estas Esperanta kombino: elsendu du signojn.
                let unua = stato.remove(0);  // Elsendi la unuan signon.
                // Kontroli ĉu la dua estas prefikso.
                if matches!(stato.chars().next().unwrap(), 
                    'c' | 'g' | 'h' | 'j' | 's' | 'u' |
                    'C' | 'G' | 'H' | 'J' | 'S' | 'U') {
                    // La dua estas prefikso, do konservu ĝin en la stato.
                } else {
                    // La dua ankaŭ ne estas prefikso, do elsendu ĝin rekte.
                    let _dua = stato.remove(0);
                }
                Some(unua)
            }
        }
    }
}
