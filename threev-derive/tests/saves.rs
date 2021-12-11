use threev_derive::Saves;

#[test]
fn test_saves() -> Result<(), anyhow::Error> {

    #[derive(Saves)]
    pub struct TestData {
        id: u32,
        name: String,
        tags: Vec<String>,
        created: u32,
    }
    #[derive(Saves)]
    struct MyTupleStruct(u32, String, i8);

    let td = TestData { id: 4, name: String::From("Chris"), tags: Vec::new(), created: 32342 };
    Ok(())

}
