// Note: Loads the contents of the module square_content from another file
//       with the same name as the module. Read more at
//       https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
mod square_content;
mod simple_board_content;
mod generic_board_content;
mod row;
mod board_index;

const BOARD_SIDE_LENGTH: usize = 10;
pub const BOARD_SIZE: usize = BOARD_SIDE_LENGTH * BOARD_SIDE_LENGTH;

// Note: Re-exports the content of the square_content module to keep paths short.
//       Read more at https://doc.rust-lang.org/reference/items/use-declarations.html#use-visibility
pub use crate::square_content::*;
pub use crate::simple_board_content::*;
pub use crate::generic_board_content::*;
pub use crate::row::*;
pub use crate::board_index::*;
