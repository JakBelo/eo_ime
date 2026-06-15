pub fn anstatauxigi(c: char, stato: &mut String) -> Option<char> {
    stato.push(c);

    match stato.as_str() {
        "cx" => { stato.clear(); return Some('ĉ'); }
        "gx" => { stato.clear(); return Some('ĝ'); }
        "hx" => { stato.clear(); return Some('ĥ'); }
        "jx" => { stato.clear(); return Some('ĵ'); }
        "sx" => { stato.clear(); return Some('ŝ'); }
        "ux" => { stato.clear(); return Some('ŭ'); }

        "c" | "g" | "h" | "j" | "s" | "u" => return None,

        _ => {
            if stato.len() > 1 {
                let first = stato.remove(0);
                return Some(first);
            }
            return Some(c);
        }
    }
}
