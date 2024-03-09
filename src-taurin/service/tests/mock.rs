mod prepare;

use service::{ MutationUser,QueryUser};
use entity::user;
use prepare::prepare_mock_db;

#[tokio::test]
async fn main() {
    let db = &prepare_mock_db();

    {
        let user = QueryUser::find_user_by_id(db, "1".to_string()).await.unwrap().unwrap();
        assert_eq!(user.id, "1".to_string());
    }
    
}
