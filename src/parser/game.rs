use std::collections::HashSet;
use strum_macros::{EnumString, VariantNames};

use super::{Info, Team};

#[derive(Clone, Copy, Debug, PartialEq, EnumString, VariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum PitchesInfo {
    Pitches,
    Count,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, EnumString, VariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum DayNightInfo {
    Day,
    Night,
}

#[derive(Clone, Copy, Debug, PartialEq, EnumString, VariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum FieldConditionInfo {
    Dry,
    Soaked,
    Wet,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, EnumString, VariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum PrecipitationInfo {
    Drizzle,
    None,
    Rain,
    Showers,
    Snow,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, EnumString, VariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum SkyInfo {
    Cloudy,
    Dome,
    Night,
    Overcast,
    Sunny,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TemperatureInfo {
    Known(u8),
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, EnumString, VariantNames)]
pub enum WindDirectionInfo {
    #[strum(serialize = "fromcf")]
    FromCenterField,
    #[strum(serialize = "fromlf")]
    FromLeftField,
    #[strum(serialize = "fromrf")]
    FromRightField,
    #[strum(serialize = "ltor")]
    LeftToRight,
    #[strum(serialize = "rtol")]
    RightToLeft,
    #[strum(serialize = "tocf")]
    ToCenterField,
    #[strum(serialize = "tolf")]
    ToLeftField,
    #[strum(serialize = "torf")]
    ToRightField,
    #[strum(serialize = "unknown")]
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WindSpeedInfo {
    Known(u8),
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TimeOfGameInfo {
    Known(u16),
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttendanceInfo {
    Known(u32),
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, EnumString, VariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum GameTypeInfo {
    Regular,
    Exhibition,
    Preseason,
    AllStar,
    Playoff,
    WorldSeries,
    #[strum(serialize = "lcs")]
    LeagueChampionshipSeries,
    DivisionSeries,
    Wildcard,
    Championship,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InfoState {
    pub visiting_team: Option<String>,
    pub home_team: Option<String>,
    pub date: Option<String>,
    pub number: Option<u8>,
    pub start_time: Option<String>,
    pub day_night: Option<DayNightInfo>,
    pub innings: Option<u8>,
    pub tiebreaker: Option<u8>,
    pub used_designated_hitter_rule: Option<bool>,
    pub pitches: Option<PitchesInfo>,
    pub official_scorer: Option<String>,
    pub home_team_bat_first: Option<bool>,
    pub umpire_home: Option<String>,
    pub umpire_1b: Option<String>,
    pub umpire_2b: Option<String>,
    pub umpire_3b: Option<String>,
    pub umpire_left_field: Option<String>,
    pub umpire_right_field: Option<String>,
    pub field_condition: Option<FieldConditionInfo>,
    pub precipitation: Option<PrecipitationInfo>,
    pub sky: Option<SkyInfo>,
    pub temperature: Option<TemperatureInfo>,
    pub wind_direction: Option<WindDirectionInfo>,
    pub wind_speed: Option<WindSpeedInfo>,
    pub time_of_game: Option<TimeOfGameInfo>,
    pub attendance: Option<AttendanceInfo>,
    pub site: Option<String>,
    pub wp: Option<String>,
    pub lp: Option<String>,
    pub save: Option<String>,
    pub game_winning_rbi: Option<String>,
    pub game_type: Option<GameTypeInfo>,
}

impl Default for InfoState {
    fn default() -> Self {
        Self {
            visiting_team: None,
            home_team: None,
            date: None,
            number: Some(0),
            start_time: None,
            day_night: None,
            innings: Some(9),
            tiebreaker: Some(0),
            used_designated_hitter_rule: None,
            pitches: None,
            official_scorer: None,
            home_team_bat_first: Some(false),
            umpire_home: None,
            umpire_1b: None,
            umpire_2b: None,
            umpire_3b: None,
            umpire_left_field: None,
            umpire_right_field: None,
            field_condition: None,
            precipitation: None,
            sky: None,
            temperature: None,
            wind_direction: None,
            wind_speed: None,
            time_of_game: None,
            attendance: None,
            site: None,
            wp: None,
            lp: None,
            save: None,
            game_winning_rbi: None,
            game_type: Some(GameTypeInfo::Regular),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Hand {
    Left,
    Right,
    Both,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumString, VariantNames)]
pub enum Position {
    #[strum(serialize = "1")]
    Pitcher,
    #[strum(serialize = "2")]
    Catcher,
    #[strum(serialize = "3")]
    FirstBase,
    #[strum(serialize = "4")]
    SecondBase,
    #[strum(serialize = "5")]
    ThirdBase,
    #[strum(serialize = "6")]
    Shortstop,
    #[strum(serialize = "7")]
    LeftField,
    #[strum(serialize = "8")]
    CenterField,
    #[strum(serialize = "9")]
    RightField,
    #[strum(serialize = "10")]
    DesignatedHitter,
    #[strum(serialize = "11")]
    PinchHitter,
    #[strum(serialize = "12")]
    PinchRunner,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub id: String,
    pub name: String,
    // batting_hand: Hand,
    // throwing_hand: Hand,
    pub team: String,
    pub positions: HashSet<Position>,
    pub batting_order: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Count {
    Known {
        balls: u8,
        strikes: u8,
    },
    Unknown,
}

#[derive(Clone, Debug, PartialEq, EnumString, VariantNames)]
pub enum PitchType {
    #[strum(serialize = "A")]
    AutomaticStrike,
    #[strum(serialize = "B")]
    Ball,
    #[strum(serialize = "C")]
    CalledStrike,
    #[strum(serialize = "F")]
    Foul,
    #[strum(serialize = "H")]
    HitBatter,
    #[strum(serialize = "I")]
    IntentionalBall,
    #[strum(serialize = "K")]
    Strike,
    #[strum(serialize = "L")]
    FoulBunt,
    #[strum(serialize = "M")]
    MissedBuntAttempt,
    #[strum(serialize = "N")]
    NoPitch,
    #[strum(serialize = "O")]
    FoulTipOnBunt,
    #[strum(serialize = "P")]
    Pitchout,
    #[strum(serialize = "Q")]
    SwingingOnPitchout,
    #[strum(serialize = "R")]
    FoulBallOnPitchout,
    #[strum(serialize = "S")]
    SwingingStrike,
    #[strum(serialize = "T")]
    FoulTip,
    #[strum(serialize = "U")]
    UnknownOrMissedPitch,
    #[strum(serialize = "V")]
    CalledBall,
    #[strum(serialize = "X")]
    BallInPlayByBatter,
    #[strum(serialize = "Y")]
    BallInPlayOnPitchout,
    #[strum(serialize = "1")]
    PickoffThrowToFirstBase,
    #[strum(serialize = "2")]
    PickoffThrowToSecondBase,
    #[strum(serialize = "3")]
    PickoffThrowToThirdBase,
    #[strum(serialize = ".")]
    NotInvolvingBatter,
}

#[derive(Clone, Debug, PartialEq, EnumString, VariantNames)]
pub enum PitchModifier {
    #[strum(serialize = "+")]
    PickoffThrowByCatcher,
    #[strum(serialize = "*")]
    BlockedByCatcher,
    #[strum(serialize = ">")]
    RunnerGoingOnPitch,
}


#[derive(Clone, Debug, PartialEq)]
pub struct Pitch {
    pub pitch_type: PitchType,
    pub pitch_modifier: Option<PitchModifier>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FieldingErrorType {
    ThrowingError(Option<Base>),
    NonThrowingError,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Fielder {
    Known(u8),
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BallPathNode {
    Success {
        fielder: Fielder,
    },
    Error {
        fielder: Fielder,
        error_type: FieldingErrorType,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, EnumString, VariantNames)]
pub enum Base {
    #[strum(serialize = "1")]
    First,
    #[strum(serialize = "2")]
    Second,
    #[strum(serialize = "3")]
    Third,
    #[strum(serialize = "H", serialize = "B")]
    Home,
}

// taken from https://www.retrosheet.org/location.htm
#[derive(Clone, Debug, PartialEq, EnumString, VariantNames)]
pub enum FieldLocation {
    // outfield
    #[strum(serialize = "78XD")]
    ExtremelyDeepCenterLeft,
    #[strum(serialize = "8XD")]
    ExtremelyDeepCenter,
    #[strum(serialize = "89XD")]
    ExtremelyDeepCenterRight,
    #[strum(serialize = "7LDF")]
    DeepLeftFoulFence,
    #[strum(serialize = "7LD")]
    DeepLeftFoul,
    #[strum(serialize = "7D")]
    DeepLeft,
    #[strum(serialize = "78D")]
    DeepCenterLeft,
    #[strum(serialize = "8D")]
    DeepCenter,
    #[strum(serialize = "89D")]
    DeepCenterRight,
    #[strum(serialize = "9D")]
    DeepRight,
    #[strum(serialize = "9LD")]
    DeepRightFoul,
    #[strum(serialize = "9LDF")]
    DeepRightFoulFence,
    #[strum(serialize = "7LF")]
    LeftFoulFence,
    #[strum(serialize = "7L")]
    LeftFoul,
    #[strum(serialize = "7")]
    Left,
    #[strum(serialize = "78")]
    CenterLeft,
    #[strum(serialize = "8")]
    Center,
    #[strum(serialize = "89")]
    CenterRight,
    #[strum(serialize = "9")]
    Right,
    #[strum(serialize = "9L")]
    RightFoul,
    #[strum(serialize = "9LF")]
    RightFoulFence,
    #[strum(serialize = "7LSF")]
    ShortLeftFoulFence,
    #[strum(serialize = "7LS")]
    ShortLeftFoul,
    #[strum(serialize = "7S")]
    ShortLeft,
    #[strum(serialize = "78S")]
    ShortCenterLeft,
    #[strum(serialize = "8S")]
    ShortCenter,
    #[strum(serialize = "89S")]
    ShortCenterRight,
    #[strum(serialize = "9S")]
    ShortRight,
    #[strum(serialize = "9LS")]
    ShortRightFoul,
    #[strum(serialize = "9LSF")]
    ShortRightFoulFence,
    // infield
    #[strum(serialize = "5DF")]
    DeepThirdBaseFence,
    #[strum(serialize = "5D")]
    DeepThirdBase,
    #[strum(serialize = "56D")]
    DeepThirdShortstop,
    #[strum(serialize = "6D")]
    DeepShortstop,
    #[strum(serialize = "6MD")]
    DeepMiddleShortstop,
    #[strum(serialize = "4MD")]
    DeepMiddleSecondBase,
    #[strum(serialize = "4D")]
    DeepSecondBase,
    #[strum(serialize = "34D")]
    DeepSecondFirstBase,
    #[strum(serialize = "3D")]
    DeepFirstBase,
    #[strum(serialize = "3DF")]
    DeepFirstBaseFence,
    #[strum(serialize = "5F")]
    ThirdBaseFence,
    #[strum(serialize = "56")]
    ThirdShortstop,
    #[strum(serialize = "6")]
    Shortstop,
    #[strum(serialize = "6M")]
    MiddleShortstop,
    #[strum(serialize = "4M")]
    MiddleSecondBase,
    #[strum(serialize = "4")]
    SecondBase,
    #[strum(serialize = "34")]
    SecondFirstBase,
    #[strum(serialize = "3")]
    FirstBase,
    #[strum(serialize = "3F")]
    FirstBaseFence,
    #[strum(serialize = "5S")]
    ShortThirdBase,
    #[strum(serialize = "56S")]
    ShortThirdShortstop,
    #[strum(serialize = "6S")]
    ShortShortstop,
    #[strum(serialize = "6MS")]
    ShortMiddleShortstop,
    #[strum(serialize = "4MS")]
    ShortMiddleSecondBase,
    #[strum(serialize = "4S")]
    ShortSecondBase,
    #[strum(serialize = "34S")]
    ShortSecondFirstBase,
    #[strum(serialize = "3S")]
    ShortFirstBase,
    #[strum(serialize = "15")]
    PitcherThird,
    #[strum(serialize = "1")]
    Pitcher,
    #[strum(serialize = "13")]
    PitcherFirst,
    #[strum(serialize = "25F")]
    CatcherThirdFence,
    #[strum(serialize = "25")]
    CatcherThird,
    #[strum(serialize = "1S")]
    ShortPitcher,
    #[strum(serialize = "23")]
    CatcherFirst,
    #[strum(serialize = "23F")]
    CatcherFirstFence,
    #[strum(serialize = "2")]
    Catcher,
    #[strum(serialize = "2F")]
    CatcherFence,
}

#[derive(Clone, Debug, PartialEq, EnumString, VariantNames)]
pub enum Runner {
    #[strum(serialize = "B")]
    Batter,
    #[strum(serialize = "1")]
    First,
    #[strum(serialize = "2")]
    Second,
    #[strum(serialize = "3")]
    Third,
}

impl Runner {
    pub fn from_fielder(fielder: &Fielder) -> Option<Self> {
        if let Fielder::Known(fielder) = fielder {
            return match fielder {
                3 => Some(Runner::Third),
                4 => Some(Runner::Second),
                5 => Some(Runner::First),
                _ => None,
            };
        }

        None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventType {
    Out {
        credited_fielder: Fielder,
        assisting_fielders: Vec<Fielder>,
        runner_out: Runner,
    },
    DoublePlay {
        credited_fielders: Vec<Fielder>,
        assisting_fielders: Vec<Fielder>,
        runners_out: Vec<Runner>,
    },
    TriplePlay {
        credited_fielders: Vec<Fielder>,
        assisting_fielders: Vec<Fielder>,
        runners_out: Vec<Runner>,
    },
    Interference,
    Single {
        credited_fielder: Fielder,
        assisting_fielders: Vec<Fielder>,
    },
    Double {
        credited_fielder: Fielder,
        assisting_fielders: Vec<Fielder>,
    },
    GroundRuleDouble,
    Triple {
        credited_fielder: Fielder,
        assisting_fielders: Vec<Fielder>,
    },
    Error {
        credited_fielder: Fielder,
        assisting_fielders: Vec<Fielder>,
    },
    FieldersChoice {
        credited_fielder: Fielder,
    },
    ErrorOnFoulFlyBall {
        credited_fielder: Fielder,
    },
    SoloHomeRun,
    InsideTheParkHomeRun {
        credited_fielder: Fielder,
    },
    HitByPitch,
    Strikeout {
        ball_path: Vec<BallPathNode>,
        base_running_event: Box<Option<EventType>>,
    },
    NoPlay,
    Walk {
        intentional: bool,
        base_running_event: Box<Option<EventType>>,
    },
    Balk,
    CaughtStealing {
        base: Base,
        ball_path: Vec<BallPathNode>,
    },
    DefensiveIndifference,
    OtherAdvance,
    PassedBall,
    WildPitch,
    Pickoff {
        caught_stealing: bool,
        base: Base,
        ball_path: Vec<BallPathNode>,
    },
    StolenBase {
        bases: Vec<Base>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventModifier {
    AppealPlay,
    PopUpBunt,
    GroundBallBunt,
    BuntGroundedIntoDoublePlay,
    BatterInterference,
    LineDriveBunt,
    BattingOutOfTurn,
    BuntPoppedIntoDoublePlay,
    RunnerHitByBattedBall,
    CalledThirdStrike,
    CourtesyBatter,
    CourtesyFielder,
    CourtesyRunner,
    UnspecifiedDoublePlay,
    Error(Fielder),
    Fly,
    FlyBallDoublePlay,
    FanInterference,
    Foul,
    ForceOut,
    GroundBall,
    GroundBallDoublePlay,
    GroundBallTriplePlay,
    InfieldFlyRule,
    Interference,
    InsideTheParkHomeRun,
    LineDrive,
    LinedIntoDoublePlay,
    LinedIntoTriplePlay,
    ManagerChallengeOfCallOnTheField,
    NoDoublePlayCreditedForThisPlay,
    FielderObstructingRunner,
    PopFly,
    RunnerPassedAnotherRunnerAndWasCalledOut,
    RelayThrowFromFielderWithNoOut(Fielder),
    RunnerInterference,
    SacrificeFly,
    SacrificeHitOrBunt,
    Throw(Option<Base>),
    UnspecifiedTriplePlay,
    UmpireInterference,
    UmpireReviewOfCallOnTheField,
    HitLocation(FieldLocation),
}

#[derive(Clone, Debug, PartialEq)]
pub enum AdvanceParameter {
    BallPath(Vec<BallPathNode>),
    Unearned,
    TeamUnearned,
    RBICredited,
    RBINotCredited,
    Interference(FieldLocation),
    WildPitch,
    PassedBall,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Advance {
    pub starting_base: Base,
    pub ending_base: Base,
    pub out: bool,
    pub parameters: Vec<AdvanceParameter>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub event_type: EventType,
    pub modifiers: Vec<EventModifier>,
    pub advances: Vec<Advance>,
}

#[derive(Clone, Debug, PartialEq, EnumString, VariantNames)]
pub enum PlayNote {
    #[strum(serialize = "#")]
    UncertainHash,
    #[strum(serialize = "!")]
    Exceptional,
    #[strum(serialize = "?")]
    UncertainQuestion,
    #[strum(serialize = "+")]
    HardHit,
    #[strum(serialize = "-")]
    SoftHit,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Play {
    info_state: InfoState,
    home_team_players: Vec<Player>,
    visiting_team_players: Vec<Player>,
    inning: u8,
    batter_id: String,
    count: Option<Count>,
    pitches: Option<Vec<Pitch>>,
    event: Option<Event>,
    note: Option<PlayNote>,
    comments: Vec<String>,
}

#[derive(Clone)]
pub struct Game {
    id: String,
    version: u8,
    plays: Vec<Play>,
}

#[derive(Clone)]
pub struct GameBuilder {
    pub info_state: InfoState,
    pub home_team_players: Vec<Player>,
    pub visiting_team_players: Vec<Player>,
    pub id: Option<String>,
    pub version: Option<u8>,
    pub plays: Vec<Play>,
}

impl GameBuilder {
    pub fn new() -> Self {
        Self {
            info_state: InfoState::default(),
            home_team_players: Vec::new(),
            visiting_team_players: Vec::new(),
            id: None,
            version: None,
            plays: Vec::new(),
        }
    }

    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn get_version(&self) -> Option<&u8> {
        self.version.as_ref()
    }

    pub fn get_info_state(&self) -> &InfoState {
        &self.info_state
    }

    pub fn get_home_team_players(&self) -> &Vec<Player> {
        &self.home_team_players
    }

    pub fn get_visiting_team_players(&self) -> &Vec<Player> {
        &self.visiting_team_players
    }

    pub fn get_plays(&self) -> &Vec<Play> {
        &self.plays
    }

    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    pub fn set_version(&mut self, version: u8) {
        self.version = Some(version);
    }

    pub fn set_info_state_visiting_team(&mut self, visiting_team: String) {
        self.info_state.visiting_team = Some(visiting_team);
    }

    pub fn set_info_state_home_team(&mut self, home_team: String) {
        self.info_state.home_team = Some(home_team);
    }

    pub fn set_info_state_date(&mut self, date: String) {
        self.info_state.date = Some(date);
    }

    pub fn set_info_state_number(&mut self, number: u8) {
        self.info_state.number = Some(number);
    }

    pub fn set_info_state_start_time(&mut self, start_time: String) {
        self.info_state.start_time = Some(start_time);
    }

    pub fn set_info_state_day_night(&mut self, day_night: DayNightInfo) {
        self.info_state.day_night = Some(day_night);
    }

    pub fn set_info_state_innings(&mut self, innings: u8) {
        self.info_state.innings = Some(innings);
    }

    pub fn set_info_state_tiebreaker(&mut self, tiebreaker: u8) {
        self.info_state.tiebreaker = Some(tiebreaker);
    }

    pub fn set_info_state_used_designated_hitter_rule(&mut self, used_designated_hitter_rule: bool) {
        self.info_state.used_designated_hitter_rule = Some(used_designated_hitter_rule);
    }

    pub fn set_info_state_pitches(&mut self, pitches: PitchesInfo) {
        self.info_state.pitches = Some(pitches);
    }

    pub fn set_info_state_official_scorer(&mut self, official_scorer: String) {
        self.info_state.official_scorer = Some(official_scorer);
    }

    pub fn set_info_state_home_team_bat_first(&mut self, home_team_bat_first: bool) {
        self.info_state.home_team_bat_first = Some(home_team_bat_first);
    }

    pub fn set_info_state_umpire_home(&mut self, umpire_home: Option<String>) {
        self.info_state.umpire_home = umpire_home;
    }

    pub fn set_info_state_umpire_1b(&mut self, umpire_1b: Option<String>) {
        self.info_state.umpire_1b = umpire_1b;
    }

    pub fn set_info_state_umpire_2b(&mut self, umpire_2b: Option<String>) {
        self.info_state.umpire_2b = umpire_2b;
    }

    pub fn set_info_state_umpire_3b(&mut self, umpire_3b: Option<String>) {
        self.info_state.umpire_3b = umpire_3b;
    }

    pub fn set_info_state_umpire_left_field(&mut self, umpire_left_field: Option<String>) {
        self.info_state.umpire_left_field = umpire_left_field;
    }

    pub fn set_info_state_umpire_right_field(&mut self, umpire_right_field: Option<String>) {
        self.info_state.umpire_right_field = umpire_right_field;
    }

    pub fn set_info_state_field_condition(&mut self, field_condition: FieldConditionInfo) {
        self.info_state.field_condition = Some(field_condition);
    }

    pub fn set_info_state_precipitation(&mut self, precipitation: PrecipitationInfo) {
        self.info_state.precipitation = Some(precipitation);
    }

    pub fn set_info_state_sky(&mut self, sky: SkyInfo) {
        self.info_state.sky = Some(sky);
    }

    pub fn set_info_state_temperature(&mut self, temperature: TemperatureInfo) {
        self.info_state.temperature = Some(temperature);
    }

    pub fn set_info_state_wind_direction(&mut self, wind_direction: WindDirectionInfo) {
        self.info_state.wind_direction = Some(wind_direction);
    }

    pub fn set_info_state_wind_speed(&mut self, wind_speed: WindSpeedInfo) {
        self.info_state.wind_speed = Some(wind_speed);
    }

    pub fn set_info_state_time_of_game(&mut self, time_of_game: TimeOfGameInfo) {
        self.info_state.time_of_game = Some(time_of_game);
    }

    pub fn set_info_state_attendance(&mut self, attendance: AttendanceInfo) {
        self.info_state.attendance = Some(attendance);
    }

    pub fn set_info_state_site(&mut self, site: String) {
        self.info_state.site = Some(site);
    }

    pub fn set_info_state_wp(&mut self, wp: String) {
        self.info_state.wp = Some(wp);
    }

    pub fn set_info_state_lp(&mut self, lp: String) {
        self.info_state.lp = Some(lp);
    }

    pub fn set_info_state_save(&mut self, save: Option<String>) {
        self.info_state.save = save;
    }

    pub fn set_info_state_game_winning_rbi(&mut self, game_winning_rbi: Option<String>) {
        self.info_state.game_winning_rbi = game_winning_rbi;
    }

    pub fn set_info_state_game_type(&mut self, game_type: GameTypeInfo) {
        self.info_state.game_type = Some(game_type);
    }

    pub fn set_info(&mut self, info: Info) {
        match info {
            Info::VisitingTeam(visiting_team) => self.set_info_state_visiting_team(visiting_team),
            Info::HomeTeam(home_team) => self.set_info_state_home_team(home_team),
            Info::Date(date) => self.set_info_state_date(date),
            Info::Number(number) => self.set_info_state_number(number),
            Info::StartTime(start_time) => self.set_info_state_start_time(start_time),
            Info::DayNight(day_night) => self.set_info_state_day_night(day_night),
            Info::Innings(innings) => self.set_info_state_innings(innings),
            Info::Tiebreaker(tiebreaker) => self.set_info_state_tiebreaker(tiebreaker),
            Info::UsedDesignatedHitterRule(used_designated_hitter_rule) => self.set_info_state_used_designated_hitter_rule(used_designated_hitter_rule),
            Info::Pitches(pitches) => self.set_info_state_pitches(pitches),
            Info::OfficialScorer(official_scorer) => self.set_info_state_official_scorer(official_scorer),
            Info::HomeTeamBatFirst(home_team_bat_first) => self.set_info_state_home_team_bat_first(home_team_bat_first),
            Info::UmpireHome(umpire_home) => self.set_info_state_umpire_home(umpire_home),
            Info::Umpire1B(umpire_1b) => self.set_info_state_umpire_1b(umpire_1b),
            Info::Umpire2B(umpire_2b) => self.set_info_state_umpire_2b(umpire_2b),
            Info::Umpire3B(umpire_3b) => self.set_info_state_umpire_3b(umpire_3b),
            Info::UmpireLeftField(umpire_left_field) => self.set_info_state_umpire_left_field(umpire_left_field),
            Info::UmpireRightField(umpire_right_field) => self.set_info_state_umpire_right_field(umpire_right_field),
            Info::FieldCondition(field_condition) => self.set_info_state_field_condition(field_condition),
            Info::Precipitation(precipitation) => self.set_info_state_precipitation(precipitation),
            Info::Sky(sky) => self.set_info_state_sky(sky),
            Info::Temperature(temperature) => self.set_info_state_temperature(temperature),
            Info::WindDirection(wind_direction) => self.set_info_state_wind_direction(wind_direction),
            Info::WindSpeed(wind_speed) => self.set_info_state_wind_speed(wind_speed),
            Info::TimeOfGame(time_of_game) => self.set_info_state_time_of_game(time_of_game),
            Info::Attendance(attendance) => self.set_info_state_attendance(attendance),
            Info::Site(site) => self.set_info_state_site(site),
            Info::WP(wp) => self.set_info_state_wp(wp),
            Info::LP(lp) => self.set_info_state_lp(lp),
            Info::Save(save) => self.set_info_state_save(save),
            Info::GameWinningRBI(game_winning_rbi) => self.set_info_state_game_winning_rbi(game_winning_rbi),
            Info::GameType(game_type) => self.set_info_state_game_type(game_type),
            Info::Other(_, _) => (),
        }
    }

    pub fn set_player(&mut self, team: &Team, player: Player) {
        match team {
            Team::Visiting => {
                let existing_player_index = self.visiting_team_players.iter().position(|p| p.id == player.id);
                if let Some(index) = existing_player_index {
                    let mut updated_player = player;
                    updated_player.positions.extend(self.visiting_team_players[index].positions.clone());
                    self.visiting_team_players[index] = updated_player;
                } else {
                    self.visiting_team_players.push(player);
                }
            },
            Team::Home => {
                let existing_player_index = self.home_team_players.iter().position(|p| p.id == player.id);
                if let Some(index) = existing_player_index {
                    let mut updated_player = player;
                    updated_player.positions.extend(self.home_team_players[index].positions.clone());
                    self.home_team_players[index] = updated_player;
                } else {
                    self.home_team_players.push(player);
                }
            },
        }
    }

    pub fn play(&mut self, play: Play) {
        self.plays.push(play);
    }

    pub fn build(self) -> Result<Game, String> {
        let id = self.id.ok_or("id is required")?;
        let version = self.version.ok_or("version is required")?;

        let plays = if self.plays.is_empty() {
            Err("plays are required".to_string())
        } else {
            Ok(self.plays)
        };

        Ok(Game {
            id,
            version,
            plays: plays?,
        })
    }
}
