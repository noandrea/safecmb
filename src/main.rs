use bip39::{Language, Mnemonic};
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

fn main() {
    // generate the mnemonic
    let random_bytes = thread_rng().gen::<[u8; 32]>();
    let mnemonic = Mnemonic::from_entropy_in(Language::English, &random_bytes)
        .unwrap()
        .word_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let (dseq, mnemonic_out) = safecmb(&mnemonic);

    // ++ print ++
    println!(
        "{:3} | {:10} || {:3} | {}",
        "seq", "word", "decoding", "new mnemonic"
    );
    println!("--- | ---------- || --- | ----------");
    for x in 0..mnemonic.len() {
        println!(
            " {:02} | {:10} || {:02}  | {:10}",
            x + 1,
            mnemonic[x],
            dseq[x] + 1,
            mnemonic_out[x],
        );
    }
}

fn safecmb(mnemonic: &Vec<String>) -> (Vec<usize>, Vec<String>) {
    // generate random key sequence
    let mut keys: Vec<usize> = (0..mnemonic.len()).collect();
    keys.shuffle(&mut thread_rng());

    // generate the new word sequence
    let words: Vec<String> = keys
        .clone()
        .iter()
        .map(|x| mnemonic[*x].to_owned())
        .collect();

    // calculate the deocoding sequence
    let words_1_index: HashMap<_, _> = words.iter().enumerate().map(|(i, w)| (w, i)).collect();
    let dseq: Vec<usize> = mnemonic
        .iter()
        .map(|w| *words_1_index.get(w).expect("that's impossible!"))
        .collect();

    (dseq, words)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_safecmb() {
        let mnemonic_in = vec![
            "merry", "exchange", "minute", "leopard", "season", "undo", "nasty", "decrease",
            "toilet", "tuna", "practice", "salt", "special", "enough", "country", "result", "frog",
            "elder", "evoke", "ride", "genius", "speed", "gaze", "timber",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let (dseq, mnemonic_out) = super::safecmb(&mnemonic_in);

        dseq.into_iter()
            .enumerate()
            .for_each(|(index, index_dseq)| {
                assert_eq!(mnemonic_in[index], mnemonic_out[index_dseq])
            })
    }
}
