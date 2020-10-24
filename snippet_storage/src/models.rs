//use diesel::backend::Backend;
 use diesel::{backend::Backend, deserialize::Queryable};
#[derive(Queryable, Debug)]
pub struct Snippet {
    pub id: i32,
    pub title: String,
    #[diesel(deserialize_as = "StringToVec")]
    pub description: Vec<String>,
    pub snippet: String,
    #[diesel(deserialize_as = "StringToVec")]
    pub mnemonics: Vec<String>,
    #[diesel(deserialize_as = "StringToVec")]
    pub tags: Vec<String>,
}
pub struct StringToVec(Vec<String>);

// impl From<StringToVec> for Vec<String> {
//     fn from(s: StringToVec) -> Vec<String> {
//         s.0
//     }
// }

impl Into<Vec<String>> for StringToVec {
    fn into(self) -> Vec<String> {
        self.0
    }
}

impl<DB, ST> Queryable<ST, DB> for StringToVec
where
    DB: Backend,
    String: Queryable<ST, DB>,
{
    type Row = <String as Queryable<ST, DB>>::Row;

    fn build(row: Self::Row) -> Self {
        StringToVec(vec![])
    }
}