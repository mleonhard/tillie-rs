type RowId = u64;

pub struct ColumnMetadata<Row, T> {}

pub struct Index<Row, T> {
    col: ColumnMetadata<Row, T>,
}
impl<Row, T> Index<Row, T> {
    pub fn new<T>(engine: &mut Engine, col: ColumnMetadata<Row, T>) -> Self {
        Self { col }
    }
}

pub struct UniqueIndex<Row, T> {
    col: ColumnMetadata<Row, T>,
}
impl<Row, T> UniqueIndex<Row, T> {
    pub fn new<T>(engine: &mut Engine, col: ColumnMetadata<Row, T>) -> Self {
        Self { col }
    }
}

pub struct IndexMetadata {}

pub struct Rows<T> {}
impl<T> Rows<T> {
    pub fn new(engine: &mut Engine) -> Self {
        Self {}
    }
}
pub struct RowsMetadata {}

#[derive(Debug)]
pub struct DbError(String);

pub struct Engine {}
impl Engine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create<T>(&mut self, table_name: &str) -> Result<T, DbError> {
        unimplemented!()
    }

    pub fn load<T>(&mut self, table_name: &str) -> Result<Option<T>, DbError> {
        unimplemented!()
    }

    pub fn migrate<A: Table, B: Table>(
        &mut self,
        a: A,
        f: fn(A::RowType) -> B::RowType,
    ) -> Result<B, DbError> {
        unimplemented!()
    }

    pub fn rename<T>(&mut self, table: T, new_table_name: &str) -> Result<T, DbError> {
        unimplemented!()
    }
}

pub trait Table {
    type RowType;
    fn indexes(&self) -> &[IndexMetadata];
}

pub trait InitTable {
    fn init(engine: &mut Engine) -> Self;
}

pub struct OwnersRow {
    pub id: u64,
    pub name: String,
}

//#[derive(tillie::Table)]
pub struct Owners {
    pub rows: Rows<OwnersRow>,
    pub by_id: UniqueIndex<OwnersRow, u64>,
    pub by_name: Index<OwnersRow, String>,
}
impl InitTable for Owners {
    fn init(engine: &mut Engine) -> Self {
        Self {
            rows: Rows::new(engine),
            by_id: UniqueIndex::new(engine, OwnersRow::id_col()),
            by_name: Index::new(engine, OwnersRow::name_col()),
        }
    }
}
impl Table for Owners {
    type RowType = OwnersRow;
    fn indexes(&self) -> &[IndexMetadata] {
        todo!()
    }
}
pub fn load_owners(engine: &mut Engine) -> Owners {
    engine
        .load("owners")
        .unwrap()
        .unwrap_or_else(|| engine.create("owners").unwrap())
}

//#[derive(tillie::Row)]
struct PetsRow0 {
    pub id: u64,
    pub name: String,
    pub legs: u8,
    pub owner: u64,
}
//#[derive(tillie::Table)]
struct Pets0 {
    pub rows: Rows<PetsRow0>,
    pub by_id: UniqueIndex<PetsRow0, u64>,
    pub by_name: Index<PetsRow0, String>,
    pub by_legs: Index<PetsRow0, u8>,
    pub by_owner: Index<PetsRow0, u64>,
}
impl InitTable for Pets0 {
    fn init(engine: &mut Engine) -> Self {
        Self {
            rows: Rows::new(engine),
            by_id: UniqueIndex::new(engine, PetsRow0::id_col()),
            by_name: Index::new(engine, PetsRow0::name_col()),
            by_legs: Index::new(engine, PetsRow0::legs_col()),
            by_owner: Index::new(engine, PetsRow0::owner_col()),
        }
    }
}
impl Table for Pets0 {
    type RowType = PetsRow0;
    fn indexes(&self) -> &[IndexMetadata] {
        todo!()
    }
}

//#[derive(tillie::Row)]
pub struct PetsRow {
    pub id: u64,
    pub name: String,
    pub leg_count: u8,
    pub owner_id: u64,
}

//#[derive(tillie::Table)]
pub struct Pets {
    pub rows: Rows<PetsRow>,
    pub by_id: UniqueIndex<PetsRow, u64>,
    pub by_name: Index<PetsRow, String>,
    pub by_leg_count: Index<PetsRow, u8>,
    pub by_owner_id: Index<PetsRow, u64>,
}
impl InitTable for Pets {
    fn init(engine: &mut Engine) -> Self {
        Self {
            rows: Rows::new(engine),
            by_id: UniqueIndex::new(engine, PetsRow::id_col()),
            by_name: Index::new(engine, PetsRow::name_col()),
            by_leg_count: Index::new(engine, PetsRow::leg_count_col()),
            by_owner_id: Index::new(engine, PetsRow::owner_col()),
        }
    }
}
impl Table for Pets {
    type RowType = PetsRow;
    fn indexes(&self) -> &[IndexMetadata] {
        todo!()
    }
}

pub fn load_pets(engine: &mut Engine) -> Pets {
    engine
        .load("pet")
        .unwrap()
        .map(|t| engine.rename(t, "pets").unwrap())
        .map(|t| {
            engine
                .migrate::<Pets0, _>(t, |row| PetsRow {
                    id: row.id,
                    name: row.name,
                    leg_count: row.legs,
                    owner_id: row.owner,
                })
                .unwrap()
        })
        .or_else(|| engine.load("pets").unwrap())
        .unwrap_or_else(|| engine.create("pets").unwrap())
}

//#[derive(Database)]
pub struct Db {
    pub engine: Engine,
    pub pets: Pets,
    pub owners: Owners,
}
pub struct TxDb {
    pub pets: Tx<Pets>,
    pub owners: Tx<Owners>,
}

pub fn test_pets() {
    let engine = Engine::new().unwrap();
    let pets = load_pets(&mut engine);
    let owners = load_owners(&mut engine);
    let db = Db {
        engine,
        pets,
        owners,
    };
    let query = db
        .pets
        .by_leg_count
        .range_inclusive(3, u8::MAX)
        .filter_map(|pets_row: PetsRow| {
            if let Some(owners_row) = db.owners.by_id.get(pets_row.owner_id) {
                (pets_row.name.clone(), owners_row.name.clone())
            } else {
                None
            }
        });
    for (pet_name, owner_name) in query {
        println!("pet={pet_name}, owner={owner_name}");
    }
    db.txn(|db: TxDb| {
        let Some(owner_row) = db.owners.by_name.get("Joe") else {
            return;
        };
        db.pets
            .by_name
            .update("Spot", |row| row.owner = owner_row.id)?;
    })
    .unwrap();
    println!("Done.");
}
