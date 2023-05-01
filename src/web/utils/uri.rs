use lazy_regex::regex_replace_all;

// pub fn encode_uri(s: impl AsRef<str>) -> String {
//     regex_replace_all!(r"[^A-Za-z0-9_\-\.:/\\]", s.as_ref(), |seq: &str| {
//         let mut r = String::new();
//         for ch in seq.to_owned().bytes() {
//             r.push('%');
//             r.push_str(octet_to_hex(ch).as_ref());
//         }
//         r.clone()
//     }).into_owned()
// }

pub fn decode_uri(s: impl AsRef<str>) -> String {
    regex_replace_all!(r"(%[A-Fa-f0-9]{2})+", s.as_ref(), |seq: &str, _| {
        let mut r = Vec::<u8>::new();
        let inp: Vec<u8> = seq.to_owned().bytes().collect();
        let mut i: usize = 0;
        while i != inp.len() {
            r.push(u8::from_str_radix(String::from_utf8_lossy(&[inp[i + 1], inp[i + 2]]).as_ref(), 16).unwrap_or(0));
            i += 3;
        }
        String::from_utf8_lossy(r.as_ref()).into_owned().to_owned()
    }).into_owned()
}

// fn octet_to_hex(arg: u8) -> String {
//     let r = format!("{:x}", arg);
//     ((if r.len() == 1 { "0" } else { "" }).to_owned() + &r).to_uppercase().to_owned()
// }