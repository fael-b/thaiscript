use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LetterVariant::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LetterVariant::Id)
                            .string_len(24)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(LetterVariant::Letter)
                            .string_len(4)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LetterVariant::LearningOrder)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LetterVariant::Category)
                            .enumeration(
                                Alias::new("category"),
                                [
                                    Alias::new("consonant"),
                                    Alias::new("vowel"),
                                    Alias::new("digit"),
                                    Alias::new("tone"),
                                    Alias::new("other"),
                                ],
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LetterVariant::Group)
                            .enumeration(
                                Alias::new("group"),
                                [
                                    Alias::new("high"),
                                    Alias::new("middle"),
                                    Alias::new("low-sonorant"),
                                    Alias::new("low-voiceless"),
                                    Alias::new("short"),
                                    Alias::new("long"),
                                    Alias::new("final"),
                                    Alias::new("diphtong"),
                                    Alias::new("quasi-letter"),
                                    Alias::new("other"),
                                ],
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LetterVariant::Position)
                            .enumeration(
                                Alias::new("position"),
                                [
                                    Alias::new("before"),
                                    Alias::new("middle"),
                                    Alias::new("end"),
                                    Alias::new("anywhere"),
                                    Alias::new("other"),
                                ],
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LetterVariant::Romanization)
                            .string_len(8)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LetterVariant::ExampleWord)
                            .string_len(16)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LetterVariant::ExampleWordExplanation)
                            .string_len(16)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LetterVariant::ExampleWordTransliteration)
                            .string_len(16)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LetterVariant::ExampleWordEmoji)
                            .string_len(16)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LetterVariant::SimilarWords)
                            .text() // Array of string (to be serialized as JSON)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create relevant indexes
        manager
            .create_index(
                Index::create()
                    .table(LetterVariant::Table)
                    .name("idx_letter_variant_category")
                    .col(LetterVariant::Category)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop relevant indexes
        manager
            .drop_index(
                Index::drop()
                    .table(LetterVariant::Table)
                    .name("idx_letter_variant_category")
                    .to_owned(),
            )
            .await?;

        // Drop table afterwards
        manager
            .drop_table(Table::drop().table(LetterVariant::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum LetterVariant {
    Table,
    Id,
    Letter,
    LearningOrder,
    Category,
    Group,
    Position,
    Romanization,
    ExampleWord,
    ExampleWordExplanation,
    ExampleWordTransliteration,
    ExampleWordEmoji,
    SimilarWords,
}
