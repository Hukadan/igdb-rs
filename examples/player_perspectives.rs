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

        let ppc = igdb_client.player_perspectives();

        //Get perspectives for Borderlands 3 (id 19164)
        let game_perspectives_ids = games_client
            .get_first_by_id(19164)
            .await
            .unwrap()
            .player_perspectives;

        for p in game_perspectives_ids {
            let perspective = ppc.get_first_by_id(p).await.unwrap();
            println!(
                "id: {}, name: {}, url: {}",
                perspective.id, perspective.name, perspective.url
            );
        }

        //id: 1, name: First person, url: https://www.igdb.com/player_perspectives/first-person
    })
}
