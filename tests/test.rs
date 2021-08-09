pub mod schema1 {
    pub struct Pet {
        pub id: u64,
        pub name: String,
        pub legs: u8,
        pub owner: String,
    }

    pub struct Owner {
        pub id: u64,
        pub name: String,
    }

    pub struct Db<'log> {
        pub pets: Table<Pet>,
        pub owners: Table<Owner>,
        pub log: Log,
    }
}

pub mod schema2 {
    pub enum PetSpecies {
        Dog,
        Cat,
    }

    pub struct Pet {
        pub id: u64,
        pub name: String,
        pub legs: u8,
        pub species: PetSpecies,
        pub birthdate: chrono::NaiveDate,
    }

    pub struct Owner {
        pub id: u64,
        pub name: String,
        pub pet_name: String,
    }

    pub enum LogEntry {
        PetEntry(Pet),
        OwnerEntry(Owner),
    }

    pub struct Db {
        pub pets: Table<Pet>,
        pub owners: Table<Owner>,
        pub log: Log,
    }
}

pub fn main() {
    let pets_data: TableData<schema1::Pet> = Table::open("pets").unwrap();
    let name_key: Unique<String, schema1::Pet> = Unique::open("name_key", |pet| pet.name).unwrap();
    let name_to_pet: Index<String, schema1::Pet> = Index::open("name_to_pet", |pet| pet.name).unwrap();
    let legs_to_pet: Index<u8, schema1::Pet> = Index::open("legs_to_pet", |pet| pet.legs).unwrap();
    let owner_name_key: Unique<String, schema1::Owner> = Unique::open("owner_name_key", |owner| owner.name).unwrap();
    let pet_owner_fkey: ForeignKey<String, schema1::Pet> = Index::open("legs_to_pet", |pet| pet.owner, &owner_name_key).unwrap();
    let pets : Table<schema1::Pet> = Table::new([&pets_data, &name_key, &name_to_pet, &legs_to_pet]);
    let mut txn = db.log.begin();
    let fido_id = db
        .pets
        .set(schema1::Pet {
                id: 0,
                name: "Fido".to_string(),
                legs: 4,
            owner: "Joe".to_string()
        },
        )
        .unwrap();
    pets.remove(&mut txn, )
    db.pets.remove(&mut txn, fido_id).unwrap();
    txn.commit().unwrap();
}
