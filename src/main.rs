use crate::names::*;
use rand::Rng;

mod names;

#[derive(Debug)]
enum FamilyMember {
    Dad { name: String },
    Mon { name: String },
    Son { name: String },
    Daughter { name: String },
    NotFamily { name: String },
}

fn guess_member(name: String) -> FamilyMember {
    match name {
        n if Vec::from(FATHER.to_vec()).contains(&n.as_str()) => FamilyMember::Dad { name: n },
        n if Vec::from(MOTHER.to_vec()).contains(&n.as_str()) => FamilyMember::Mon { name: n },
        n if Vec::from(SON.to_vec()).contains(&n.as_str()) => FamilyMember::Son { name: n },
        n if Vec::from(DAUGHTER.to_vec()).contains(&n.as_str()) => {
            FamilyMember::Daughter { name: n }
        }
        n => FamilyMember::NotFamily { name: n },
    }
}

fn identify_threat(subject: &FamilyMember) -> String {
    if let FamilyMember::NotFamily { name: _ } = subject {
        format!("{:?} is a dangerous barbarian!", subject)
    } else {
        format!("{:?} is family! ♥", subject)
    }
}

fn main() {
    let mut all_possible_names = Vec::new();
    all_possible_names.append(&mut FATHER.to_vec());
    all_possible_names.append(&mut MOTHER.to_vec());
    all_possible_names.append(&mut SON.to_vec());
    all_possible_names.append(&mut DAUGHTER.to_vec());
    all_possible_names.append(&mut BARBARIANS.to_vec());

    let mut rng = rand::thread_rng();
    let subject = guess_member(String::from(
        all_possible_names
            .get(rng.gen_range(0, all_possible_names.len()))
            .expect("There must be a name choosed")
            .clone(),
    ));
    println!("Guess: {:?}", &subject);
    print!("{}", identify_threat(&subject));
}
