#[allow(warnings)]
pub mod prisma;
pub mod prisma_ext;

use async_once::AsyncOnce;
use lazy_static::lazy_static;
use prisma::{excerpt, image, PrismaClient};

lazy_static! {
    pub static ref PRISMA_CLIENT: AsyncOnce<PrismaClient> =
        AsyncOnce::new(async { prisma::new_client().await.unwrap() });
}

macro_rules! table_helper {
    ($($table:ident),*) => {
        paste::paste! {
            $(
                pub async fn [<$table s>]<'a>() -> $table::Actions<'a> {
                    PRISMA_CLIENT.get().await.$table()
                }
            )*
        }
    }
}

table_helper!(excerpt, image);
