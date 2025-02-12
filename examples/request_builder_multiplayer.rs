use async_std::task;
use igdb::client::IGDBClient;

fn main() {
    task::block_on(async {
        use std::env;
        let client_id =
            env::var("IGDB_CLIENT_ID").expect("You nee to set the IGDB_CLIENT_ID variable");
        let token = env::var("IGDB_TOKEN").expect("You nee to set the IGDB_TOKEN variable");
        let igdb_client = IGDBClient::new(&client_id, &token);

        let games_client = igdb_client.games();

        let mut games_req = IGDBClient::create_request();
        games_req
            .add_field("id")
            .add_field("name")
            .search("Borderlands 2")
            .limit(3);

        let result = games_client.get(games_req).await.unwrap();

        let ids: Vec<String> = result.iter().map(|g| g.id.to_string()).collect();
        let names: Vec<String> = result.iter().map(|g| g.name.clone()).collect();

        let multiplayer_client = igdb_client.multiplayer_modes();

        let mut mul_request = IGDBClient::create_request();

        mul_request.all_fields().add_where_in("id".to_owned(), ids);

        let results = multiplayer_client.get(mul_request).await.unwrap();

        for (i, m) in results.iter().enumerate() {
            println!("{} has online coop: {}", names[i], m.onlinecoop);
            println!("{} has local coop: {}", names[i], m.lancoop);
        }

        //  Prints:
        //  Borderlands 2 has online coop: true
        //  Borderlands 2 has local coop: false
    })
}
