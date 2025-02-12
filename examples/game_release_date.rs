use async_std::task;
use igdb::client::IGDBClient;

fn main() {
    task::block_on(async {
        use std::env;
        let client_id =
            env::var("IGDB_CLIENT_ID").expect("You nee to set the IGDB_CLIENT_ID variable");
        let token = env::var("IGDB_TOKEN").expect("You nee to set the IGDB_TOKEN variable");
        let igdb_client = IGDBClient::new(&client_id, &token);

        let release_client = igdb_client.release_dates();

        //Get releases for Borderlands3 with id 19164
        let releases = release_client.get_by_game_id(19164, 10).await.unwrap();

        let platform_client = igdb_client.platforms();

        for release in releases {
            let platform = platform_client
                .get_first_by_id(release.platform as usize)
                .await
                .unwrap();

            println!(
                "platform: {} release date: {}",
                platform.name, release.human
            );
        }

        //  platform: Xbox One release date: 2019-Sep-13
        //  platform: PC (Microsoft Windows) release date: 2019-Sep-13
        //  platform: PlayStation 4 release date: 2019-Sep-13
        //  platform: Google Stadia release date: 2019-Sep-13
    })
}
