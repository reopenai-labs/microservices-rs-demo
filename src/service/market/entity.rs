
pub mod symbol {
    use sea_orm::entity::prelude::*;
    use serde::Serialize;
    
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
    #[sea_orm(table_name = "symbol")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i64,
        pub code: String,
    }
    
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    
    impl ActiveModelBehavior for ActiveModel {}
}


