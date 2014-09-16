#![license = "MIT"]
#![deny(missing_doc)]
#![deny(warnings)]

//! Zippers for walking and editing data structures.

pub trait Zipper<Direction> {
    /// Moves the Zipper to a new location. Should return false if the move
    /// is impossible.
    fn go(&mut self, Direction) -> bool;
}

pub trait Accessor<Data, Direction>: Zipper<Direction> {
    /// Get an immutable reference to the data at this point.
    fn focus(&self) -> &Data;
}

pub trait AccessorMut<Data, Direction>: Accessor<Data, Direction> {
    /// Get a mutable reference to the data at this point.
    fn focus_mut(&mut self) -> &mut Data;
}

pub trait Editor<Data, Direction, Cntx>: Zipper<Direction>
where Cntx: Context<Data, Direction, Self> {
    /// Try to remove the data at this point, possibly creating a hole.
    fn remove(self, Direction) -> Result<(Data, Cntx), Self>;

    /// Move the data at this point in the specified direction, creating a hole.
    fn shove(self, Direction) -> Cntx;
}

pub trait Context<Data, Direction, Edtr>: Zipper<Direction>
where Edtr: Editor<Data, Direction, Self> {
    /// Insert data in the hole in this context, closing it.
    fn insert(self, Data) -> Edtr;

    /// Move the hole in the specified direction, shifting the rest
    /// of the data and possibly plugging the hole.
    fn plug(self, Direction) -> Result<Edtr, Self>;
}

/// A data structure which can be edited using Context's and Editor's.
pub trait Editable<Data, Direction, Edtr, Cntx>
where Edtr: Editor<Data, Direction, Cntx>,
      Cntx: Context<Data, Direction, Edtr> {
    /// Edit this data structure through an Editor
    fn deconstruct(self) -> Edtr;

    /// Reconstruct this data structure from an Editor
    fn reconstruct(Edtr) -> Self;
}

