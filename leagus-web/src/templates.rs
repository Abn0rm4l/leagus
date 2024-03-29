use askama::Template;
use leagus::models::{League, Participant, PointsTable, Season, Session, Venue};

#[derive(Template)]
#[template(path = "leagues.html")]
pub struct LeaguesFullTemplate {
    pub leagues: Vec<League>,
}

#[derive(Template)]
#[template(path = "partials/leagues/single_content.html")]
pub struct LeagueContentTemplate {
    pub league: League,
    pub seasons: Vec<Season>,
    pub active_season: Option<Season>,
    pub active_session: Option<Session>,
    pub points_table: PointsTable,
}

#[derive(Template)]
#[template(path = "league.html")]
pub struct LeagueTemplate {
    pub league: League,
    pub seasons: Vec<Season>,
    pub active_season: Option<Season>,
    pub active_session: Option<Session>,
    pub points_table: PointsTable,
}

#[derive(Template)]
#[template(path = "participants.html")]
pub struct ParticipantsTemplate {
    pub participants: Vec<Participant>,
}

#[derive(Template)]
#[template(path = "partials/participants/create_participant.html")]
pub struct CreateParticipantTemplate {}

#[derive(Template)]
#[template(path = "partials/seasons/create_season.html")]
pub struct SeasonCreateModalTemplate {
    pub league: League,
}

#[derive(Template)]
#[template(path = "venues.html")]
pub struct VenuesTemplate {
    pub venues: Vec<Venue>,
}

#[derive(Template)]
#[template(path = "partials/venues/create_venue.html")]
pub struct CreateVenueTemplate {}
