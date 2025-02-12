use crate::model::enums::PlatformCategory;

#[derive(Deserialize, Debug, Clone)]
pub struct Platform {
    #[serde(default)]
    pub id: usize,
    #[serde(default)]
    pub abbreviation: String,
    #[serde(default)]
    pub alternative_name: String,
    #[serde(default)]
    pub category: PlatformCategory,
    #[serde(default)]
    pub created_at: u64,
    #[serde(default)]
    pub generation: usize,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub platform_logo: usize,
    #[serde(default)]
    pub platform_family: usize,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub versions: Vec<u32>,
    #[serde(default)]
    pub websites: Vec<u32>,
}
