use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(File::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(File::SavedFileName)
                            .string()
                            .not_null()
                            .primary_key(),  // UUID v7 문자열을 primary key로 설정
                    )
                    .col(ColumnDef::new(File::OriginalFileName).string().not_null())
                    .col(ColumnDef::new(File::FilePath).string().not_null())
                    .col(
                        ColumnDef::new(File::MimeType)
                            .string()
                            .not_null()
                            .default("application/octet-stream")
                    )
                    .col(ColumnDef::new(File::Size).big_integer().not_null())
                    .col(ColumnDef::new(File::Hash).string().not_null())
                    .col(
                        ColumnDef::new(File::IsUse)
                            .boolean()
                            .not_null()
                            .default(true)
                    )
                    .col(
                        ColumnDef::new(File::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .col(
                        ColumnDef::new(File::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .col(ColumnDef::new(File::Description).string())  // optional
                    .col(ColumnDef::new(File::UploadedBy).string())   // optional
                    .col(ColumnDef::new(File::Category).string())     // optional
                    .col(ColumnDef::new(File::Metadata).json())       // optional
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(File::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum File {
    Table,
    SavedFileName,    // UUID v7 primary key
    OriginalFileName, // 원본 파일명
    FilePath,         // 파일 경로
    MimeType,         // MIME 타입
    Size,             // 파일 크기
    Hash,             // 파일 해시
    IsUse,            // 사용 여부
    CreatedAt,        // 생성 시간
    UpdatedAt,        // 수정 시간
    Description,      // 파일 설명 (optional)
    UploadedBy,       // 업로드한 사용자 (optional)
    Category,         // 카테고리 (optional)
    Metadata,         // 메타데이터 JSON (optional)
}
