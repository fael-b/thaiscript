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
                            .string_len(24)
                            .not_null(),
                    )
                    .col(ColumnDef::new(ReviewOutcome::Date).date_time().not_null())
                    .col(ColumnDef::new(ReviewOutcome::Outcome).string().not_null())
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
    Outcome,
    MsTimeTaken,
}
