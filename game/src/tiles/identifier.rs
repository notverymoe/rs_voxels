// Copyright 2024 Natalie Baker // AGPLv3 //

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileIdentifier(u16);

impl TileIdentifier {
    pub const DEFAULT: TileIdentifier = TileIdentifier(0);
}
