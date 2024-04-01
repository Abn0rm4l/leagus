use askama::Template;
use leagus::models::{League, Match, Participant, PointsTable, Round, Season, Session, Venue};

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
    pub points_table_template: PointsTableTemplate,
    pub session_template: SessionTemplate,
}

#[derive(Template)]
#[template(path = "league.html")]
pub struct LeagueTemplate {
    pub league_content_template: LeagueContentTemplate,
}

#[derive(Template)]
#[template(path = "seasons_for_league.html")]
pub struct SeasonsForLeagueTemplate {
    pub league: League,
    pub seasons: Vec<Season>,
    /// Just used for the breadcrumbs
    pub active_season: Option<Season>,
}

#[derive(Template)]
#[template(path = "partials/seasons/points_table.html")]
pub struct PointsTableTemplate {
    pub points_table: PointsTable,
}

#[derive(Template)]
#[template(path = "partials/sessions/single_session.html")]
pub struct SessionTemplate {
    pub active_session: Option<Session>,
    pub active_season: Option<Season>,
}

//TODO: Give these better/clearer names
#[derive(Template)]
#[template(path = "partials/sessions/session_view.html")]
pub struct SessionViewTemplate {
    pub session: Session,
    pub active_round: Option<Round>,
    pub rounds: Vec<Round>,
}

#[derive(Template)]
#[template(path = "partials/rounds/round_view.html")]
pub struct RoundViewTemplate {
    pub session: Session,
    pub active_round: Option<Round>,
    pub rounds: Vec<Round>,
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
