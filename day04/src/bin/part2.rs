use day04::password;
use day04::password::MatchingPairStrategy;

fn main() {
    let passwords = password::find_valid_passwords_in_range(165432, 707912, &MatchingPairStrategy::PAIR_ONLY);

    println!("found {}: {:?}", passwords.len(), passwords);
}
