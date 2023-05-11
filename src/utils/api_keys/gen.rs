use rand::Rng;

// Generate a random API Key.
pub fn generate_api_key(length: usize) -> String {
    let mut output: Vec<String> = Vec::with_capacity(length);
    let mut rand_gen = rand::thread_rng();

    for _ in 0..length {
        let rand_ascii_dec = rand_gen.gen_range(39..=89);
        output.push(char::from_u32(rand_ascii_dec).unwrap().to_string());
    }

    return output.join("");
}
