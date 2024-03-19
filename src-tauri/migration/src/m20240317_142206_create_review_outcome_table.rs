use sea_orm_migration::prelude::*;

use crate::m20240316_104818_create_letter_variant_table::LetterVariant;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ReviewOutcome::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ReviewOutcome::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ReviewOutcome::LetterVariantId)
                            .string_len(24)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ReviewOutcome::ReviewType)
                            .enumeration(
                                Alias::new("review_type"),
                                [
                                    Alias::new("initial"),
                                    Alias::new("initial-drawing"),
                                    Alias::new("letter-to-romanization"),
                                    Alias::new("letter-to-speech"),
                                    Alias::new("letter-to-drawing"),
                                    Alias::new("speech-to-letter"),
                                    Alias::new("speech-to-word"),
                                    Alias::new("speech-to-transliteration"),
                                    Alias::new("speech-to-emoji"),
                                    Alias::new("speech-to-drawing"),
                                    Alias::new("emoji-to-word"),
                                    Alias::new("emoji-to-drawing"),
                                    Alias::new("romanization-to-letter"),
                                    Alias::new("word-to-emoji"),
                                ],
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(ReviewOutcome::Date).date_time().not_null())
                    .col(ColumnDef::new(ReviewOutcome::Correct).boolean().not_null())
                    .col(
                        ColumnDef::new(ReviewOutcome::MsTimeTaken)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_review_outcome_letter_variant_id")
                            .from(ReviewOutcome::Table, ReviewOutcome::LetterVariantId)
                            .to(LetterVariant::Table, LetterVariant::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_review_outcome_letter_variant_id")
                    .table(ReviewOutcome::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(ReviewOutcome::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ReviewOutcome {
    Table,
    Id,
    LetterVariantId,
    ReviewType,
    Date,
    Correct,
    MsTimeTaken,
}
