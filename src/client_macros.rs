macro_rules! create_client {
    ($i: ident, $j: ident, $k: ident) => {
        pub struct $i {
            endpoint_client: EndpointClient,
        }

        impl $i {
            ///Receives a configured RequestBuilder an returns a result containing a collection
            pub async fn get(&self, request_builder: RequestBuilder) -> Result<Vec<$j>, Error> {
                self.endpoint_client.get::<$j>(request_builder).await
            }
            ///Returns a collection filtered by id and limits the retrieved registries using limit parameter value.
            pub async fn get_by_id(&self, id: usize, limit: usize) -> Result<Vec<$j>, Error> {
                let mut request = RequestBuilder::new();
                request
                    .all_fields()
                    .add_where("id", Equality::Equal, id.to_string())
                    .limit(limit);

                self.get(request).await
            }
            /// Returns the element by Id for this client in Option<T> format.
            pub async fn get_first_by_id(&self, id: usize) -> Result<$j, Error> {
                match self.get_by_id(id, 1).await {
                    Ok(ref d) if !d.is_empty() => Ok(d[0].clone()),
                    Ok(_) => Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Empty response from server for query with id: {}", id),
                    )
                    .into()),
                    Err(e) => {
                        log::error!("{}", e);
                        Err(e)
                    }
                }
            }
        }

        impl IGDBClient {
            /// Returns a reference to the client
            pub fn $k(&self) -> $i {
                $i {
                    endpoint_client: EndpointClient::new(
                        self.client_id.clone(),
                        self.token.clone(),
                        Endpoint::$k,
                    ),
                }
            }
        }
    };
}

macro_rules! expand_get_by_game_id {
    ($i: ident, $j: ident) => {
        impl $i {
            ///Receives a game_id and a limit of registries and returns an Option<Vec> of elements
            pub async fn get_by_game_id(&self, game_id: usize, limit: usize) -> Option<Vec<$j>> {
                let mut request = RequestBuilder::new();
                request
                    .all_fields()
                    .add_where("game", Equality::Equal, game_id.to_string())
                    .limit(limit);

                match self.get(request).await {
                    Ok(d) => Some(d),
                    Err(e) => {
                        log::error!("{}", e);
                        None
                    }
                }
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! request {
    () => {
        IGDBClient::create_request()
    };
}

macro_rules! use_client_imports {
    () => {
        use crate::{
            endpoint_client::EndpointClient, endpoints::Endpoint, media_quality::MediaQuality,
            model::age_rating::AgeRating, model::artwork::Artwork, model::character::Character,
            model::character_mug_shot::CharacterMugshot, model::company::Company,
            model::cover::Cover, model::engine::Engine, model::external_game::ExternalGame,
            model::franchise::Franchise, model::game_mode::GameMode, model::game_video::GameVideo,
            model::games::Game, model::multiplayer_mode::MultiplayerMode,
            model::platform::Platform, model::platform_logo::PlatformLogo,
            model::player_perspective::PlayerPerspective, model::release_date::ReleaseDate,
            model::screenshot::Screenshot, model::theme::Theme, model::website::Website,
            request_builder::Equality, request_builder::RequestBuilder,
        };

        use crate::Error;
    };
}
