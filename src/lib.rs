use mixes_db::{Class, Database, SteamID};

pub struct MixesElo<D>
{
    db: D,
}

impl<D> MixesElo<D>
where
    D: Database,
{
    /// Create a new mixes elo calculator based on the stat database provided.
    pub fn new(db: D) -> Self { Self { db } }

    /// Calculate the win rate of the player with the given steam id. If a class
    /// is set, only the wins and losses of games where the player has played
    /// that class are included. If the class is `None`, all recent games of the
    /// player, regardless of class are considered. `num_games` is the number of
    /// games to check. The contributions in these games are not weighted, but
    /// it only considers `num_games` most recent games, or less, in case not
    /// enough logs are available.
    ///
    /// # Returns
    /// The win/games rate of the player as a fraction between 0 and 1. Higher
    /// is better. If there are no games on record, 0.5 will be returned.
    pub fn winrate(
        &mut self,
        player: SteamID,
        class: Option<Class>,
        num_games: usize,
    ) -> Result<f32, D::Error>
    {
        todo!()
    }

    /// Get access to the internal database.
    pub fn database(&self) -> &D { &self.db }
    /// Get access to the internal database mutably. The database will not be
    /// automatically updated, so this should be used whenever deemed necessary.
    pub fn database_mut(&mut self) -> &mut D { &mut self.db }
}
