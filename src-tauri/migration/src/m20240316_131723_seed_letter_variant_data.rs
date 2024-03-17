use entity::letter_variant;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::entity::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Read and parse seeding data
        let raw_seeding_data = include_str!("./seed-data-2024_03_16.json");
        let seeding_data: SeedDataContainer = serde_json::from_str(raw_seeding_data)
            .map_err(|err| DbErr::Migration(err.to_string()))?;

        // Seed the table
        let db = manager.get_connection();
        for letter_info in seeding_data.letters {
            for variant in letter_info.variants {
                let variant_id = format!("{}-{}", letter_info.id, variant.position);
                let similar_words = serde_json::to_string(&letter_info.similar_words)
                    .map_err(|err| DbErr::Migration(err.to_string()))?;
                letter_variant::ActiveModel {
                    id: Set(variant_id),
                    letter: Set(letter_info.letter.to_owned()),
                    learning_order: Set(letter_info.learning_order),
                    category: Set(letter_info.category.to_owned()),
                    group: Set(letter_info.group.to_owned()),
                    position: Set(variant.position.to_owned()),
                    romanization: Set(variant.romanization.to_owned()),
                    example_word: Set(variant.example_word.to_owned()),
                    example_word_explanation: Set(variant.example_word_explanation.to_owned()),
                    example_word_transliteration: Set(variant
                        .example_word_transliteration
                        .to_owned()),
                    example_word_emoji: Set(variant.example_word_emoji.to_owned()),
                    similar_words: Set(similar_words),
                }
                .insert(db)
                .await?;
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        letter_variant::Entity::delete_many().exec(db).await?;
        Ok(())
    }
}

#[derive(serde::Deserialize)]
struct SeedDataContainer {
    letters: Vec<LetterInfoSeed>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct LetterInfoSeed {
    id: String,
    letter: String,
    learning_order: i32,
    category: String,
    group: String,
    similar_words: Vec<String>,
    variants: Vec<LetterVariantSeed>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct LetterVariantSeed {
    position: String,
    romanization: String,
    example_word: String,
    example_word_explanation: String,
    example_word_transliteration: String,
    example_word_emoji: String,
}
