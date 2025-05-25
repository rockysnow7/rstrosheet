mod game;

use chumsky::prelude::*;
use strum::VariantNames;
use strum_macros::{EnumString};
use std::collections::HashSet;
use game::{Advance, AdvanceParameter, AttendanceInfo, BallPathNode, Base, Count, DayNightInfo, Event, EventModifier, EventType, FieldConditionInfo, FieldLocation, Fielder, FieldingErrorType, Game, GameBuilder, GameTypeInfo, Pitch, PitchModifier, PitchType, PitchesInfo, Play, PlayNote, Player, Position, PrecipitationInfo, Runner, SkyInfo, TemperatureInfo, TimeOfGameInfo, WindDirectionInfo, WindSpeedInfo};

#[derive(Debug, PartialEq, Clone, EnumString)]
enum Team {
    #[strum(serialize = "0")]
    Visiting,
    #[strum(serialize = "1")]
    Home,
}

#[derive(Debug, PartialEq, Clone)]
enum Info {
    VisitingTeam(String),
    HomeTeam(String),
    Date(String),
    Number(u8),
    StartTime(String),
    DayNight(DayNightInfo),
    Innings(u8),
    Tiebreaker(u8),
    UsedDesignatedHitterRule(bool),
    Pitches(PitchesInfo),
    OfficialScorer(String),
    HomeTeamBatFirst(bool),
    UmpireHome(Option<String>),
    Umpire1B(Option<String>),
    Umpire2B(Option<String>),
    Umpire3B(Option<String>),
    UmpireLeftField(Option<String>),
    UmpireRightField(Option<String>),
    FieldCondition(FieldConditionInfo),
    Precipitation(PrecipitationInfo),
    Sky(SkyInfo),
    Temperature(TemperatureInfo),
    WindDirection(WindDirectionInfo),
    WindSpeed(WindSpeedInfo),
    TimeOfGame(TimeOfGameInfo),
    Attendance(AttendanceInfo),
    Site(String),
    WP(String),
    LP(String),
    Save(Option<String>),
    GameWinningRBI(Option<String>),
    GameType(GameTypeInfo),
    Other(String, String),
}

#[derive(Debug, PartialEq)]
enum Line {
    Id(String),
    Version(u8),
    StartSub {
        is_start: bool,
        player_id: String,
        player_name: String,
        team: Team,
        batting_order: u8,
        position: Position,
    },
    Info(Info),
    Play {
        inning: u8,
        team: Team,
        batter_id: String,
        count: Count,
        pitches: Option<Vec<Pitch>>,
        event: Event,
        note: Option<PlayNote>,
    },
}

impl Line {
    fn any_one_or_more<'a>() -> impl Parser<'a, &'a str, String, extra::Err<Rich<'a, char>>> {
        none_of(",")
            .repeated()
            .at_least(1)
            .collect::<String>()
    }
    
    fn number<'a>() -> impl Parser<'a, &'a str, usize, extra::Err<Rich<'a, char>>> {
        any()
            .filter(char::is_ascii_digit)
            .repeated()
            .at_least(1)
            .collect::<String>()
            .map(|digits| digits.parse::<usize>().unwrap())
    }

    fn boolean<'a>() -> impl Parser<'a, &'a str, bool, extra::Err<Rich<'a, char>>> {
        just("true")
            .to(true)
            .or(just("false").to(false))
    }

    fn parse_id<'a>() -> impl Parser<'a, &'a str, Self, extra::Err<Rich<'a, char>>> {
        just("id,")
            .ignore_then(any().repeated().at_least(1).collect::<String>())
            .map(|id| Self::Id(id))
    }

    fn parse_version<'a>() -> impl Parser<'a, &'a str, Self, extra::Err<Rich<'a, char>>> {
        just("version,")
            .ignore_then(Self::number())
            .map(|version| Self::Version(version as u8))
    }

    fn parse_start_sub<'a>() -> impl Parser<'a, &'a str, Self, extra::Err<Rich<'a, char>>> {
        just("start")
            .to(true)
            .or(just("sub").to(false))
            .then_ignore(just(","))
            .then(Self::any_one_or_more())
            .then_ignore(just(","))
            .then(Self::any_one_or_more())
            .then_ignore(just(","))
            .then(one_of("01").map(|c: char| c.to_string().parse::<Team>().unwrap()))
            .then_ignore(just(","))
            .then(one_of('0'..='9').map(|c: char| c.to_digit(10).unwrap() as u8))
            .then_ignore(just(","))
            .then(any()
                .repeated()
                .at_least(1)
                .at_most(2)
                .collect::<String>()
                .filter(|s| Position::VARIANTS.contains(&s.as_str()))
                .map(|position| position.parse::<Position>().unwrap()))
            .map(|(((((is_start, player_id), player_name), team), batting_order), position)| Self::StartSub {
                is_start,
                player_id,
                player_name,
                team,
                batting_order,
                position,
            })
    }

    fn parse_info<'a>() -> impl Parser<'a, &'a str, Self, extra::Err<Rich<'a, char>>> {
        let team_name = any()
            .filter(|c: &char| 'A' <= *c && *c <= 'Z')
            .repeated()
            .at_least(1)
            .at_most(3)
            .collect::<String>();

        let visiting_team = just("visteam,")
            .ignore_then(team_name)
            .map(|visteam| Self::Info(Info::VisitingTeam(visteam)));

        let home_team = just("hometeam,")
            .ignore_then(team_name)
            .map(|hometeam| Self::Info(Info::HomeTeam(hometeam)));

        let date = just("date,")
            .ignore_then(Self::any_one_or_more())
            .map(|date| Self::Info(Info::Date(date)));

        let number = just("number,")
            .ignore_then(Self::number())
            .map(|number| Self::Info(Info::Number(number as u8)));

        let start_time = just("starttime,")
            .ignore_then(Self::any_one_or_more())
            .map(|start_time| Self::Info(Info::StartTime(start_time)));

        let day_night = just("daynight,")
            .ignore_then(Self::any_one_or_more()
                .filter(|s| DayNightInfo::VARIANTS.contains(&s.as_str())))
            .map(|day_night| Self::Info(Info::DayNight(day_night.parse().unwrap())));

        let innings = just("innings,")
            .ignore_then(Self::number())
            .map(|innings| Self::Info(Info::Innings(innings as u8)));

        let tiebreaker = just("tiebreaker,")
            .ignore_then(any().filter(|c: &char| "123".contains(*c)))
            .map(|tiebreaker| Self::Info(Info::Tiebreaker(tiebreaker.to_digit(10).unwrap() as u8)));

        let used_designated_hitter_rule = just("usedh,")
            .ignore_then(Self::boolean())
            .map(|usedh| Self::Info(Info::UsedDesignatedHitterRule(usedh)));

        let pitches = just("pitches,")
            .ignore_then(Self::any_one_or_more()
                .filter(|s| PitchesInfo::VARIANTS.contains(&s.as_str())))
            .map(|pitches| Self::Info(Info::Pitches(pitches.parse().unwrap())));

        let official_scorer = just("oscorer,")
            .ignore_then(Self::any_one_or_more())
            .map(|official_scorer| Self::Info(Info::OfficialScorer(official_scorer)));

        let home_team_bat_first = just("htbf,")
            .ignore_then(Self::boolean())
            .map(|home_team_bat_first| Self::Info(Info::HomeTeamBatFirst(home_team_bat_first)));

        let umpire_home = just("umphome,")
            .ignore_then(just("(none)").to(None).or(Self::any_one_or_more().map(Some)))
            .map(|umpire_home| Self::Info(Info::UmpireHome(umpire_home)));

        let umpire_1b = just("ump1b,")
            .ignore_then(just("(none)").to(None).or(Self::any_one_or_more().map(Some)))
            .map(|umpire_1b| Self::Info(Info::Umpire1B(umpire_1b)));

        let umpire_2b = just("ump2b,")
            .ignore_then(just("(none)").to(None).or(Self::any_one_or_more().map(Some)))
            .map(|umpire_2b| Self::Info(Info::Umpire2B(umpire_2b)));

        let umpire_3b = just("ump3b,")
            .ignore_then(just("(none)").to(None).or(Self::any_one_or_more().map(Some)))
            .map(|umpire_3b| Self::Info(Info::Umpire3B(umpire_3b)));

        let umpire_left_field = just("umplf,")
            .ignore_then(just("(none)").to(None).or(Self::any_one_or_more().map(Some)))
            .map(|umpire_left_field| Self::Info(Info::UmpireLeftField(umpire_left_field)));

        let umpire_right_field = just("umprf,")
            .ignore_then(just("(none)").to(None).or(Self::any_one_or_more().map(Some)))
            .map(|umpire_right_field| Self::Info(Info::UmpireRightField(umpire_right_field)));

        let field_condition = just("fieldcond,")
            .ignore_then(Self::any_one_or_more()
                .filter(|s| FieldConditionInfo::VARIANTS.contains(&s.as_str())))
            .map(|field_condition| Self::Info(Info::FieldCondition(field_condition.parse().unwrap())));

        let precipitation = just("precip,")
            .ignore_then(Self::any_one_or_more()
                .filter(|s| PrecipitationInfo::VARIANTS.contains(&s.as_str())))
            .map(|precipitation| Self::Info(Info::Precipitation(precipitation.parse().unwrap())));

        let sky = just("sky,")
            .ignore_then(Self::any_one_or_more()
                .filter(|s| SkyInfo::VARIANTS.contains(&s.as_str())))
            .map(|sky| Self::Info(Info::Sky(sky.parse().unwrap())));

        let temperature = just("temp,")
            .ignore_then(Self::number()
                .map(|temperature| match temperature {
                    0 => TemperatureInfo::Unknown,
                    _ => TemperatureInfo::Known(temperature as u8),
                }))
            .map(|temperature| Self::Info(Info::Temperature(temperature)));

        let wind_direction = just("winddir,")
            .ignore_then(Self::any_one_or_more()
                .filter(|s| WindDirectionInfo::VARIANTS.contains(&s.as_str())))
            .map(|wind_direction| Self::Info(Info::WindDirection(wind_direction.parse().unwrap())));
        
        let wind_speed = just("windspeed,")
            .ignore_then(just("-1")
                .to(WindSpeedInfo::Unknown)
                .or(Self::number().map(|wind_speed| WindSpeedInfo::Known(wind_speed as u8))))
            .map(|wind_speed| Self::Info(Info::WindSpeed(wind_speed)));

        let time_of_game = just("timeofgame,")
            .ignore_then(Self::number()
                .map(|time_of_game| match time_of_game {
                    0 => TimeOfGameInfo::Unknown,
                    _ => TimeOfGameInfo::Known(time_of_game as u16),
                }))
            .map(|time_of_game| Self::Info(Info::TimeOfGame(time_of_game)));

        let attendance = just("attendance,")
            .ignore_then(Self::number()
                .map(|attendance| match attendance {
                    0 => AttendanceInfo::Unknown,
                    _ => AttendanceInfo::Known(attendance as u32),
                }))
            .map(|attendance| Self::Info(Info::Attendance(attendance)));

        let site = just("site,")
            .ignore_then(Self::any_one_or_more())
            .map(|site| Self::Info(Info::Site(site)));

        let wp = just("wp,")
            .ignore_then(Self::any_one_or_more())
            .map(|wp| Self::Info(Info::WP(wp)));

        let lp = just("lp,")
            .ignore_then(Self::any_one_or_more())
            .map(|lp| Self::Info(Info::LP(lp)));

        let save = just("save,")
            .ignore_then(Self::any_one_or_more().or_not())
            .map(|save| Self::Info(Info::Save(save)));

        let game_winning_rbi = just("gwrbi,")
            .ignore_then(Self::any_one_or_more().or_not())
            .map(|gwrbi| Self::Info(Info::GameWinningRBI(gwrbi)));

        let game_type = just("gametype,")
            .ignore_then(Self::any_one_or_more()
                .filter(|s| GameTypeInfo::VARIANTS.contains(&s.as_str())))
            .map(|gametype| Self::Info(Info::GameType(gametype.parse().unwrap())));

        let other = Self::any_one_or_more()
            .filter(|s| !["visteam", "hometeam", "date", "number", "starttime", "daynight", "innings", "tiebreaker", "usedh", "pitches", "oscorer", "htbf", "umphome", "ump1b", "ump2b", "ump3b", "umplf", "umprf", "fieldcond", "precip", "sky", "temp", "winddir", "windspeed", "timeofgame", "attendance", "site", "wp", "lp", "save", "gwrbi", "gametype"].contains(&s.as_str()))
            .then_ignore(just(","))
            .then(Self::any_one_or_more())
            .map(|(key, value)| Self::Info(Info::Other(key, value)));

        let one_of_info = visiting_team
            .or(home_team)
            .or(date)
            .or(number)
            .or(start_time)
            .or(day_night)
            .or(innings)
            .or(tiebreaker)
            .or(used_designated_hitter_rule)
            .or(pitches)
            .or(official_scorer)
            .or(home_team_bat_first)
            .or(umpire_home)
            .or(umpire_1b)
            .or(umpire_2b)
            .or(umpire_3b)
            .or(umpire_left_field)
            .or(umpire_right_field)
            .or(field_condition)
            .or(precipitation)
            .or(sky)
            .or(temperature)
            .or(wind_direction)
            .or(wind_speed)
            .or(time_of_game)
            .or(attendance)
            .or(site)
            .or(wp)
            .or(lp)
            .or(save)
            .or(game_winning_rbi)
            .or(game_type)
            .or(other);

        just("info,")
            .ignore_then(one_of_info)
    }

    fn pitch<'a>() -> impl Parser<'a, &'a str, Pitch, extra::Err<Rich<'a, char>>> {
        one_of(PitchModifier::VARIANTS.concat())
            .map(|c: char| c.to_string().parse::<PitchModifier>().unwrap())
            .or_not()
            .then(one_of(PitchType::VARIANTS.concat())
                .map(|c: char| c.to_string().parse::<PitchType>().unwrap()))
            .map(|(pitch_modifier, pitch_type)| Pitch {
                pitch_type,
                pitch_modifier,
            })
    }

    fn fielder<'a>() -> impl Parser<'a, &'a str, Fielder, extra::Err<Rich<'a, char>>> {
        one_of('1'..='9')
            .map(|c: char| Fielder::Known(c.to_digit(10).unwrap() as u8))
            .or(just("U").to(Fielder::Unknown))
    }

    fn runner<'a>() -> impl Parser<'a, &'a str, char, extra::Err<Rich<'a, char>>> {
        one_of(Runner::VARIANTS.concat())
    }

    fn base<'a>() -> impl Parser<'a, &'a str, Base, extra::Err<Rich<'a, char>>> {
        one_of(Base::VARIANTS.concat())
            .map(|c: char| c.to_string().parse::<Base>().unwrap())
    }

    fn parenthesised_runner<'a>() -> impl Parser<'a, &'a str, Runner, extra::Err<Rich<'a, char>>> {
        Self::runner()
            .delimited_by(just("("), just(")"))
            .map(|runner: char| runner.to_string().parse::<Runner>().unwrap())
    }

    fn error<'a>() -> impl Parser<'a, &'a str, EventType, extra::Err<Rich<'a, char>>> {
        Self::fielder()
            .repeated()
            .collect::<Vec<Fielder>>()
            .then_ignore(just("E"))
            .then(Self::fielder())
            .map(|(assisting_fielders, credited_fielder)| EventType::Error {
                assisting_fielders,
                credited_fielder,
            })
    }

    fn stolen_base<'a>() -> impl Parser<'a, &'a str, EventType, extra::Err<Rich<'a, char>>> {
        just("SB")
            .ignore_then(Self::base())
            .then(just(";")
                .ignore_then(just("SB"))
                .ignore_then(Self::base())
                .repeated()
                .collect::<Vec<Base>>())
            .map(|(initial_base, other_bases)| {
                let mut bases = vec![initial_base];
                bases.extend(other_bases);

                EventType::StolenBase { bases }
            })
    }

    fn ball_path<'a>() -> impl Parser<'a, &'a str, Vec<BallPathNode>, extra::Err<Rich<'a, char>>> {
        let throwing_modifier = just("/TH")
            .ignore_then(Self::base().or_not())
            .map(|base| FieldingErrorType::ThrowingError(base));

        let success_node = Self::fielder()
            .map(|fielder| BallPathNode::Success { fielder });

        let error_node = just("E")
            .ignore_then(Self::fielder())
            .then(throwing_modifier.or_not())
            .map(|(fielder, throwing_modifier)| {
                let error_type = match throwing_modifier {
                    Some(throwing_modifier) => throwing_modifier,
                    None => FieldingErrorType::NonThrowingError,
                };

                BallPathNode::Error {
                    fielder,
                    error_type,
                }
            });

        let path = success_node
            .repeated()
            .collect::<Vec<BallPathNode>>()
            .then(error_node.or_not())
            .map(|(success_nodes, error_node)| {
                let mut ball_path = success_nodes;
                if let Some(error_node) = error_node {
                    ball_path.push(error_node);
                }

                ball_path
            });

        path.delimited_by(just("("), just(")"))
    }

    fn caught_stealing<'a>() -> impl Parser<'a, &'a str, EventType, extra::Err<Rich<'a, char>>> {
        just("CS")
            .ignore_then(Self::base())
            .then(Self::ball_path())
            .map(|(base, ball_path)| EventType::CaughtStealing { base, ball_path })
    }

    fn other_advance<'a>() -> impl Parser<'a, &'a str, EventType, extra::Err<Rich<'a, char>>> {
        just("OA")
            .to(EventType::OtherAdvance)
    }

    fn pickoff_no_caught_stealing<'a>() -> impl Parser<'a, &'a str, EventType, extra::Err<Rich<'a, char>>> {
        just("PO")
            .ignore_then(Self::base())
            .then(Self::ball_path())
            .map(|(base, ball_path)| EventType::Pickoff {
                caught_stealing: false,
                base,
                ball_path,
            })
    }

    fn pickoff_caught_stealing<'a>() -> impl Parser<'a, &'a str, EventType, extra::Err<Rich<'a, char>>> {
        just("POCS")
            .ignore_then(Self::base())
            .then(Self::ball_path())
            .map(|(base, ball_path)| EventType::Pickoff {
                caught_stealing: true,
                base,
                ball_path,
            })
    }

    fn passed_ball<'a>() -> impl Parser<'a, &'a str, EventType, extra::Err<Rich<'a, char>>> {
        just("PB")
            .to(EventType::PassedBall)
    }

    fn wild_pitch<'a>() -> impl Parser<'a, &'a str, EventType, extra::Err<Rich<'a, char>>> {
        just("WP")
            .to(EventType::WildPitch)
    }

    fn strikeout_or_walk_event<'a>() -> impl Parser<'a, &'a str, EventType, extra::Err<Rich<'a, char>>> {
        Self::stolen_base()
            .or(Self::caught_stealing())
            .or(Self::other_advance())
            .or(Self::pickoff_no_caught_stealing())
            .or(Self::passed_ball())
            .or(Self::wild_pitch())
            .or(Self::error())
    }

    fn event_type<'a>() -> impl Parser<'a, &'a str, EventType, extra::Err<Rich<'a, char>>> {
        let out = Self::fielder()
            .repeated()
            .collect::<Vec<Fielder>>()
            .then(Self::fielder())
            .then(Self::parenthesised_runner().or_not())
            .map(|((assisting_fielders, credited_fielder), runner)| {
                let runner_out = match runner {
                    Some(runner) => runner,
                    None => Runner::from_fielder(&credited_fielder).unwrap(),
                };

                EventType::Out {
                    credited_fielder,
                    assisting_fielders,
                    runner_out,
                }
            });

        let double_play = Self::fielder()
            .repeated()
            .collect::<Vec<Fielder>>()
            .then(Self::fielder())
            .then(Self::parenthesised_runner())
            .then(Self::fielder()
                .repeated()
                .collect::<Vec<Fielder>>())
            .then(Self::fielder())
            .then(Self::parenthesised_runner().or_not())
            .map(|(((((assisting_fielders_1, credited_fielder_1), runner_1), assisting_fielders_2), credited_fielder_2), runner_2)| {
                let assisting_fielders = [assisting_fielders_1, assisting_fielders_2].concat();

                let mut runners_out = vec![runner_1];
                if let Some(runner_2) = runner_2 {
                    runners_out.push(runner_2);
                } else {
                    runners_out.push(Runner::from_fielder(&credited_fielder_2).unwrap());
                }

                let credited_fielders = vec![credited_fielder_1, credited_fielder_2];

                EventType::DoublePlay {
                    assisting_fielders,
                    runners_out,
                    credited_fielders,
                }
            });

        let triple_play = Self::fielder()
            .repeated()
            .collect::<Vec<Fielder>>()
            .then(Self::fielder())
            .then(Self::parenthesised_runner())
            .then(Self::fielder()
                .repeated()
                .collect::<Vec<Fielder>>()
                .then(Self::fielder())
                .then(Self::parenthesised_runner()))
            .then(Self::fielder()
                .repeated()
                .collect::<Vec<Fielder>>()
                .then(Self::fielder())
                .then(Self::parenthesised_runner().or_not()))
            .map(|((((assisting_fielders_1, credited_fielder_1), runner_1), ((assisting_fielders_2, credited_fielder_2), runner_2)), ((assisting_fielders_3, credited_fielder_3), runner_3))| {
                let assisting_fielders = [assisting_fielders_1, assisting_fielders_2, assisting_fielders_3].concat();

                let mut runners_out = vec![runner_1, runner_2];
                if let Some(runner_3) = runner_3 {
                    runners_out.push(runner_3);
                } else {
                    runners_out.push(Runner::from_fielder(&credited_fielder_3).unwrap());
                }

                let credited_fielders = vec![credited_fielder_1, credited_fielder_2, credited_fielder_3];

                EventType::TriplePlay {
                    assisting_fielders,
                    runners_out,
                    credited_fielders,
                }
            });

        let interference = just("C")
            .to(EventType::Interference);

        let single = just("S")
            .ignore_then(Self::fielder()
                .repeated()
                .collect::<Vec<Fielder>>())
            .then(Self::fielder().or_not())
            .map(|(assisting_fielders, credited_fielder)| {
                let credited_fielder = match credited_fielder {
                    Some(credited_fielder) => credited_fielder,
                    None => Fielder::Unknown,
                };

                EventType::Single { credited_fielder, assisting_fielders }
            });

        let double = just("D")
            .ignore_then(Self::fielder()
                .repeated()
                .collect::<Vec<Fielder>>())
            .then(Self::fielder().or_not())
            .map(|(assisting_fielders, credited_fielder)| {
                let credited_fielder = match credited_fielder {
                    Some(credited_fielder) => credited_fielder,
                    None => Fielder::Unknown,
                };

                EventType::Double { credited_fielder, assisting_fielders }
            });

        let triple = just("T")
            .ignore_then(Self::fielder()
                .repeated()
                .collect::<Vec<Fielder>>())
            .then(Self::fielder().or_not())
            .map(|(assisting_fielders, credited_fielder)| {
                let credited_fielder = match credited_fielder {
                    Some(credited_fielder) => credited_fielder,
                    None => Fielder::Unknown,
                };

                EventType::Triple { credited_fielder, assisting_fielders }
            });

        let ground_rule_double = just("DGR")
            .to(EventType::GroundRuleDouble);

        let fielders_choice = just("FC")
            .ignore_then(Self::fielder())
            .map(|credited_fielder| EventType::FieldersChoice { credited_fielder });

        let error_on_foul_fly_ball = just("FLE")
            .ignore_then(Self::fielder())
            .map(|credited_fielder| EventType::ErrorOnFoulFlyBall { credited_fielder });

        let solo_home_run = just("H")
            .or(just("HR"))
            .to(EventType::SoloHomeRun);

        let inside_the_park_home_run = just("H")
            .or(just("HR"))
            .ignore_then(Self::fielder())
            .map(|credited_fielder| EventType::InsideTheParkHomeRun { credited_fielder });

        let hit_by_pitch = just("HP")
            .to(EventType::HitByPitch);

        let strikeout = just("K")
            .ignore_then(Self::fielder()
                .repeated()
                .collect::<Vec<Fielder>>())
            .map(|fielders| fielders
                .iter()
                .map(|fielder| BallPathNode::Success { fielder: *fielder })
                .collect::<Vec<BallPathNode>>())
            .then(just("+")
                .ignore_then(Self::strikeout_or_walk_event())
                .or_not())
            .map(|(ball_path, strikeout_event): (Vec<BallPathNode>, Option<EventType>)| EventType::Strikeout {
                ball_path,
                base_running_event: Box::new(strikeout_event),
            });

        let no_play = just("NP")
            .to(EventType::NoPlay);

        let walk = just("I")
            .or(just("IW"))
            .or(just("W"))
            .then(just("+")
                .ignore_then(Self::strikeout_or_walk_event())
                .or_not())
            .map(|(intentional, strikeout_or_walk_event)| EventType::Walk {
                intentional: intentional.contains("I"),
                base_running_event: Box::new(strikeout_or_walk_event),
            });

        let balk = just("BK")
            .to(EventType::Balk);

        let defensive_indifference = just("DI")
            .to(EventType::DefensiveIndifference);

        let pickoff = Self::pickoff_no_caught_stealing()
            .or(Self::pickoff_caught_stealing());

        out
            .or(double_play)
            .or(triple_play)
            .or(interference)
            .or(single)
            .or(double)
            .or(triple)
            .or(ground_rule_double)
            .or(Self::error())
            .or(fielders_choice)
            .or(error_on_foul_fly_ball)
            .or(solo_home_run)
            .or(inside_the_park_home_run)
            .or(hit_by_pitch)
            .or(strikeout)
            .or(no_play)
            .or(walk)
            .or(balk)
            .or(Self::caught_stealing())
            .or(defensive_indifference)
            .or(Self::other_advance())
            .or(Self::passed_ball())
            .or(Self::wild_pitch())
            .or(pickoff)
            .or(Self::stolen_base())
    }

    fn event_modifier<'a>() -> impl Parser<'a, &'a str, EventModifier, extra::Err<Rich<'a, char>>> {
        let appeal_play = just("AP")
            .to(EventModifier::AppealPlay);

        let pop_up_bunt = just("BP")
            .to(EventModifier::PopUpBunt);

        let ground_ball_bunt = just("BG")
            .to(EventModifier::GroundBallBunt);

        let bunt_grounded_into_double_play = just("BGDP")
            .to(EventModifier::BuntGroundedIntoDoublePlay);

        let batter_interference = just("BINT")
            .to(EventModifier::BatterInterference);

        let line_drive_bunt = just("BL")
            .to(EventModifier::LineDriveBunt);

        let batting_out_of_turn = just("BOOT")
            .to(EventModifier::BattingOutOfTurn);

        let bunt_popped_into_double_play = just("BPDP")
            .to(EventModifier::BuntPoppedIntoDoublePlay);

        let runner_hit_by_batted_ball = just("BR")
            .to(EventModifier::RunnerHitByBattedBall);

        let called_third_strike = just("C")
            .to(EventModifier::CalledThirdStrike);

        let courtesy_batter = just("COUB")
            .to(EventModifier::CourtesyBatter);

        let courtesy_fielder = just("COUF")
            .to(EventModifier::CourtesyFielder);

        let courtesy_runner = just("COUR")
            .to(EventModifier::CourtesyRunner);

        let unspecified_double_play = just("DP")
            .to(EventModifier::UnspecifiedDoublePlay);

        let error = just("E")
            .ignore_then(Self::fielder())
            .map(|fielder| EventModifier::Error(fielder));

        let fly = just("F")
            .to(EventModifier::Fly);

        let fly_ball_double_play = just("FDP")
            .to(EventModifier::FlyBallDoublePlay);

        let fan_interference = just("FINT")
            .to(EventModifier::FanInterference);

        let foul = just("FL")
            .to(EventModifier::Foul);

        let force_out = just("FO")
            .to(EventModifier::ForceOut);

        let ground_ball = just("G")
            .to(EventModifier::GroundBall);

        let ground_ball_double_play = just("GDP")
            .to(EventModifier::GroundBallDoublePlay);

        let ground_ball_triple_play = just("GTP")
            .to(EventModifier::GroundBallTriplePlay);

        let infield_fly_rule = just("IF")
            .to(EventModifier::InfieldFlyRule);

        let interference = just("INT")
            .to(EventModifier::Interference);

        let inside_the_park_home_run = just("IPHR")
            .to(EventModifier::InsideTheParkHomeRun);

        let line_drive = just("L")
            .to(EventModifier::LineDrive);

        let lined_into_double_play = just("LDP")
            .to(EventModifier::LinedIntoDoublePlay);
        
        let lined_into_triple_play = just("LTP")
            .to(EventModifier::LinedIntoTriplePlay);

        let manager_challenge_of_call_on_the_field = just("MREV")
            .to(EventModifier::ManagerChallengeOfCallOnTheField);

        let no_double_play_credited_for_this_play = just("NDP")
            .to(EventModifier::NoDoublePlayCreditedForThisPlay);

        let fielder_obstructing_runner = just("OBS")
            .to(EventModifier::FielderObstructingRunner);

        let pop_fly = just("P")
            .to(EventModifier::PopFly);

        let runner_passed_another_runner_and_was_called_out = just("PR")
            .to(EventModifier::RunnerPassedAnotherRunnerAndWasCalledOut);

        let relay_throw_from_fielder_with_no_out = just("R")
            .ignore_then(Self::fielder())
            .map(|fielder| EventModifier::RelayThrowFromFielderWithNoOut(fielder));

        let runner_interference = just("RINT")
            .to(EventModifier::RunnerInterference);
        
        let sacrifice_fly = just("SF")
            .to(EventModifier::SacrificeFly);
        
        let sacrifice_hit_or_bunt = just("SH")
            .to(EventModifier::SacrificeHitOrBunt);
        
        let throw = just("TH")
            .ignore_then(Self::base().or_not())
            .map(|fielder| EventModifier::Throw(fielder));
        
        let unspecified_triple_play = just("TP")
            .to(EventModifier::UnspecifiedTriplePlay);
        
        let umpire_interference = just("UINT")
            .to(EventModifier::UmpireInterference);
        
        let umpire_review_of_call_on_the_field = just("UREV")
            .to(EventModifier::UmpireReviewOfCallOnTheField);

        let hit_location = one_of("123456789")
            .repeated()
            .at_least(1)
            .collect::<String>()
            .then(one_of("XDLFSM")
                .repeated()
                .collect::<String>())
            .map(|(a, b)| [a, b].concat())
            .map(|hit_location| EventModifier::HitLocation(hit_location.parse().unwrap()));

        let event_modifier = hit_location
            .or(called_third_strike)
            .or(fly)
            .or(ground_ball)
            .or(line_drive)
            .or(pop_fly)
            .or(appeal_play)
            .or(pop_up_bunt)
            .or(ground_ball_bunt)
            .or(line_drive_bunt)
            .or(runner_hit_by_batted_ball)
            .or(unspecified_double_play)
            .or(error)
            .or(foul)
            .or(force_out)
            .or(infield_fly_rule)
            .or(relay_throw_from_fielder_with_no_out)
            .or(sacrifice_fly)
            .or(sacrifice_hit_or_bunt)
            .or(throw)
            .or(unspecified_triple_play)
            .or(fly_ball_double_play)
            .or(ground_ball_double_play)
            .or(ground_ball_triple_play)
            .or(interference)
            .or(lined_into_double_play)
            .or(lined_into_triple_play)
            .or(no_double_play_credited_for_this_play)
            .or(fielder_obstructing_runner)
            .or(bunt_grounded_into_double_play)
            .or(batter_interference)
            .or(batting_out_of_turn)
            .or(bunt_popped_into_double_play)
            .or(courtesy_batter)
            .or(courtesy_fielder)
            .or(courtesy_runner)
            .or(fan_interference)
            .or(inside_the_park_home_run)
            .or(manager_challenge_of_call_on_the_field)
            .or(runner_passed_another_runner_and_was_called_out)
            .or(runner_interference)
            .or(umpire_interference)
            .or(umpire_review_of_call_on_the_field);

        just("/")
            .or_not()
            .ignore_then(event_modifier)
    }

    fn advance<'a>() -> impl Parser<'a, &'a str, Advance, extra::Err<Rich<'a, char>>> {
        let out = just("-")
            .to(false)
            .or(just("X")
                .to(true));

        let ball_path = Self::ball_path()
            .map(|ball_path| AdvanceParameter::BallPath(ball_path));

        let unearned = just("UR")
            .to(AdvanceParameter::Unearned);

        let team_unearned = just("TUR")
            .to(AdvanceParameter::TeamUnearned);

        let rbi_credited = just("RBI")
            .to(AdvanceParameter::RBICredited);

        let rbi_not_credited = just("NR")
            .or(just("NORBI"))
            .to(AdvanceParameter::RBINotCredited);

        let position = one_of('1'..'9')
            .map(|c: char| c.to_string().parse::<FieldLocation>().unwrap());

        let interference = position
            .then_ignore(just("/INT"))
            .map(|position| AdvanceParameter::Interference(position));

        let wild_pitch = just("WP")
            .to(AdvanceParameter::WildPitch);

        let passed_ball = just("PB")
            .to(AdvanceParameter::PassedBall);

        let advance_parameter = ball_path
            .or(unearned)
            .or(team_unearned)
            .or(rbi_credited)
            .or(rbi_not_credited)
            .or(wild_pitch)
            .or(passed_ball)
            .or(interference)
            .delimited_by(just("("), just(")"));

        let advance_parameters = advance_parameter
            .repeated()
            .collect::<Vec<AdvanceParameter>>();

        Self::base()
            .then(out)
            .then(Self::base())
            .then(advance_parameters)
            .map(|(((starting_base, out), ending_base), parameters)| Advance {
                starting_base,
                out,
                ending_base,
                parameters,
            })
    }

    fn parse_play<'a>() -> impl Parser<'a, &'a str, Self, extra::Err<Rich<'a, char>>> {
        let count = just("??")
            .to(Count::Unknown)
            .or(one_of("0123")
                .then(one_of("012"))
                .map(|(balls, strikes): (char, char)| Count::Known {
                    balls: balls.to_digit(10).unwrap() as u8,
                    strikes: strikes.to_digit(10).unwrap() as u8,
                }));

        let pitches = Self::pitch()
            .repeated()
            .at_least(1)
            .collect::<Vec<Pitch>>()
            .or_not();

        let event_modifiers = Self::event_modifier()
            .repeated()
            .collect::<Vec<EventModifier>>();

        let advances_body = Self::advance()
            .then(just(";").ignore_then(Self::advance()).repeated().collect::<Vec<Advance>>())
            .map(|(advance, advances)| [vec![advance], advances].concat());

        let advances = just(".")
            .ignore_then(advances_body)
            .or_not()
            .map(|advances| advances.unwrap_or_default());

        let event = Self::event_type()
            .then(event_modifiers)
            .then(advances)
            .map(|((event_type, event_modifiers), advances)| Event {
                event_type,
                modifiers: event_modifiers,
                advances,
            });

        just("play,")
            .ignore_then(Self::number().filter(|&n| n > 0).map(|n| n as u8))
            .then_ignore(just(","))
            .then(one_of("01").map(|c: char| c.to_string().parse::<Team>().unwrap()))
            .then_ignore(just(","))
            .then(Self::any_one_or_more())
            .then_ignore(just(","))
            .then(count)
            .then_ignore(just(","))
            .then(pitches)
            .then_ignore(just(","))
            .then(event)
            .then(one_of("#!?+-").map(|note: char| note.to_string().parse().unwrap()).or_not())
            .map(|((((((inning, team), batter_id), count), pitches), event), note)| Self::Play {
                inning,
                team,
                batter_id,
                count,
                pitches,
                event,
                note,
            })
    }

    fn parse_internal<'a>() -> impl Parser<'a, &'a str, Self, extra::Err<Rich<'a, char>>> {
        Self::parse_id()
            .or(Self::parse_version())
            .or(Self::parse_start_sub())
            .or(Self::parse_info())
            .or(Self::parse_play())
    }

    pub fn parse(line: &str) -> Result<Self, String> {
        let result = Self::parse_internal().parse(line);

        let errors = result.errors().collect::<Vec<_>>();
        if errors.is_empty() {
            println!("No errors");
        } else {
            println!("{} errors", errors.len());
            for error in errors {
                println!("Error: {error}");
            }
        }

        let result_output = result.into_output();

        result_output.ok_or(format!("Invalid line: {line}"))
    }
}

struct GameParser {
    game_builder: GameBuilder,
}

impl GameParser {
    pub fn new() -> Self {
        Self {
            game_builder: GameBuilder::new(),
        }
    }

    fn parse_line(&mut self, line: &Line) -> Result<(), String> {
        match line {
            Line::Id(id) => {
                self.game_builder.set_id(id.to_string());
                Ok(())
            },
            Line::Version(version) => {
                self.game_builder.set_version(*version);
                Ok(())
            },
            Line::StartSub { is_start, player_id, player_name, team, batting_order, position: fielding_position } => {
                let team_name = match team {
                    Team::Home => self.game_builder
                        .info_state
                        .home_team
                        .as_ref()
                        .ok_or("home team name not set yet".to_string())?,
                    Team::Visiting => self.game_builder
                        .info_state
                        .visiting_team
                        .as_ref()
                        .ok_or("visiting team name not set yet".to_string())?,
                };

                let player = Player {
                    id: player_id.to_string(),
                    name: player_name.to_string(),
                    team: team_name.to_string(),
                    batting_order: *batting_order,
                    positions: HashSet::from([fielding_position.clone()]),
                };
                
                self.game_builder.set_player(&team, player.clone());

                Ok(())
            },
            Line::Info(info) => {
                self.game_builder.set_info(info.clone());
                Ok(())
            },
            Line::Play { inning, team, batter_id, count, pitches, event, note } => todo!(),
        }
    }

    pub fn parse_game(&mut self, game: &str) -> Result<Game, String> {
        let lines = game.split("\n").collect::<Vec<&str>>();
        for line in lines {
            let line = Line::parse(line)?;
            self.parse_line(&line)?;
        }

        self.game_builder.clone().build()
    }
}

pub struct FileParser;

impl FileParser {
    pub fn new() -> Self {
        Self {}
    }

    fn split_games(&self, file_content: &str) -> Vec<String> {
        // split into games, where each game begins with "id,"
        file_content
            .split("id,")
            .map(|s| format!("id,{s}"))
            .collect::<Vec<String>>()
    }

    fn parse_game(&self, game: &str) -> Result<Game, String> {
        let mut parser = GameParser::new();
        parser.parse_game(game)
    }

    pub fn parse_file(&mut self, file_content: &str) -> Result<Vec<Game>, String> {
        let games = self.split_games(file_content);
        let mut parsed_games = Vec::new();
        for game in games {
            let game = self.parse_game(&game)?;
            parsed_games.push(game);
        }
        Ok(parsed_games)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod line {
        use super::*;

        #[test]
        fn parse_valid_id() {
            let line = "id,ANA201004050";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Id("ANA201004050".to_string())));
        }

        #[test]
        fn parse_version() {
            // test valid
            let line = "version,2";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Version(2)));

            // test invalid
            let line = "version,a";
            let result = Line::parse(line);
            assert!(result.is_err());
        }

        #[test]
        fn parse_info_visteam() {
            let line = "info,visteam,ANA";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::VisitingTeam("ANA".to_string()))));
        }

        #[test]
        fn parse_info_hometeam() {
            let line = "info,hometeam,LAA";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::HomeTeam("LAA".to_string()))));
        }

        #[test]
        fn parse_info_date() {
            let line = "info,date,2010-04-05";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Date("2010-04-05".to_string()))));
        }

        #[test]
        fn parse_info_number() {
            // test valid
            let line = "info,number,1";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Number(1))));

            // test invalid
            let line = "info,number,a";
            let result = Line::parse(line);
            assert!(result.is_err());
        }

        #[test]
        fn parse_info_daynight() {
            // test valid
            let line = "info,daynight,day";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::DayNight(DayNightInfo::Day))));

            let line = "info,daynight,night";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::DayNight(DayNightInfo::Night))));

            // test invalid
            let line = "info,daynight,a";
            let result = Line::parse(line);
            assert!(result.is_err());
        }

        #[test]
        fn parse_info_innings() {
            // test valid
            let line = "info,innings,9";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Innings(9))));

            // test invalid
            let line = "info,innings,a";
            let result = Line::parse(line);
            assert!(result.is_err());
        }

        #[test]
        fn parse_info_tiebreaker() {
            // test valid
            let line = "info,tiebreaker,1";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Tiebreaker(1))));

            // test invalid
            let line = "info,tiebreaker,a";
            let result = Line::parse(line);
            assert!(result.is_err());
        }

        #[test]
        fn parse_info_usedh() {
            // test valid
            let line = "info,usedh,true";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::UsedDesignatedHitterRule(true))));

            let line = "info,usedh,false";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::UsedDesignatedHitterRule(false))));

            // test invalid
            let line = "info,usedh,a";
            let result = Line::parse(line);
            assert!(result.is_err());
        }

        #[test]
        fn parse_info_pitches() {
            // test valid
            let line = "info,pitches,pitches";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Pitches(PitchesInfo::Pitches))));

            let line = "info,pitches,count";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Pitches(PitchesInfo::Count))));

            let line = "info,pitches,none";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Pitches(PitchesInfo::None))));

            // test invalid
            let line = "info,pitches,a";
            let result = Line::parse(line);
            assert!(result.is_err());
        }

        #[test]
        fn parse_info_oscorer() {
            let line = "info,oscorer,abc";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::OfficialScorer("abc".to_string()))));
        }

        #[test]
        fn parse_info_htbf() {
            // test valid
            let line = "info,htbf,true";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::HomeTeamBatFirst(true))));

            let line = "info,htbf,false";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::HomeTeamBatFirst(false))));

            // test invalid
            let line = "info,htbf,a";
            let result = Line::parse(line);
            assert!(result.is_err());
        }

        #[test]
        fn parse_info_umpire_home() {
            let line = "info,umphome,abc";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::UmpireHome(Some("abc".to_string())))));

            let line = "info,umphome,(none)";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::UmpireHome(None))));
        }

        #[test]
        fn parse_info_umpire_1b() {
            let line = "info,ump1b,abc";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Umpire1B(Some("abc".to_string())))));

            let line = "info,ump1b,(none)";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Umpire1B(None))));
        }

        #[test]
        fn parse_info_umpire_2b() {
            let line = "info,ump2b,abc";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Umpire2B(Some("abc".to_string())))));

            let line = "info,ump2b,(none)";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Umpire2B(None))));
        }

        #[test]
        fn parse_info_umpire_3b() {
            let line = "info,ump3b,abc";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Umpire3B(Some("abc".to_string())))));

            let line = "info,ump3b,(none)";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Umpire3B(None))));
        }

        #[test]
        fn parse_info_umpire_lf() {
            let line = "info,umplf,abc";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::UmpireLeftField(Some("abc".to_string())))));

            let line = "info,umplf,(none)";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::UmpireLeftField(None))));
        }

        #[test]
        fn parse_info_umpire_rf() {
            let line = "info,umprf,abc";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::UmpireRightField(Some("abc".to_string())))));

            let line = "info,umprf,(none)";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::UmpireRightField(None))));
        }

        #[test]
        fn parse_info_fieldcond() {
            // test valid
            let line = "info,fieldcond,dry";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::FieldCondition(FieldConditionInfo::Dry))));

            let line = "info,fieldcond,soaked";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::FieldCondition(FieldConditionInfo::Soaked))));

            let line = "info,fieldcond,wet";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::FieldCondition(FieldConditionInfo::Wet))));

            let line = "info,fieldcond,unknown";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::FieldCondition(FieldConditionInfo::Unknown))));

            // test invalid
            let line = "info,fieldcond,a";
            let result = Line::parse(line);
            assert!(result.is_err());
        }

        #[test]
        fn parse_info_precip() {
            // test valid
            let line = "info,precip,drizzle";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Precipitation(PrecipitationInfo::Drizzle))));

            let line = "info,precip,none";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Precipitation(PrecipitationInfo::None))));

            let line = "info,precip,rain";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Precipitation(PrecipitationInfo::Rain))));

            let line = "info,precip,showers";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Precipitation(PrecipitationInfo::Showers))));

            let line = "info,precip,snow";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Precipitation(PrecipitationInfo::Snow))));

            let line = "info,precip,unknown";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Precipitation(PrecipitationInfo::Unknown))));

            // test invalid
            let line = "info,precip,a";
            let result = Line::parse(line);
            assert!(result.is_err());
        }

        #[test]
        fn parse_info_sky() {
            // test valid
            let line = "info,sky,cloudy";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Sky(SkyInfo::Cloudy))));

            let line = "info,sky,dome";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Sky(SkyInfo::Dome))));

            let line = "info,sky,night";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Sky(SkyInfo::Night))));

            let line = "info,sky,overcast";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Sky(SkyInfo::Overcast))));

            let line = "info,sky,sunny";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Sky(SkyInfo::Sunny))));

            let line = "info,sky,unknown";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Sky(SkyInfo::Unknown))));

            // test invalid
            let line = "info,sky,a";
            let result = Line::parse(line);
            assert!(result.is_err());
        }

        #[test]
        fn parse_info_temp() {
            // test valid
            let line = "info,temp,70";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Temperature(TemperatureInfo::Known(70)))));

            let line = "info,temp,0";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Temperature(TemperatureInfo::Unknown))));

            // test invalid
            let line = "info,temp,a";
            let result = Line::parse(line);
            assert!(result.is_err());
        }

        #[test]
        fn parse_info_winddir() {
            // test valid
            let line = "info,winddir,fromcf";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::WindDirection(WindDirectionInfo::FromCenterField))));

            let line = "info,winddir,fromlf";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::WindDirection(WindDirectionInfo::FromLeftField))));

            let line = "info,winddir,fromrf";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::WindDirection(WindDirectionInfo::FromRightField))));

            let line = "info,winddir,ltor";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::WindDirection(WindDirectionInfo::LeftToRight))));

            let line = "info,winddir,rtol";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::WindDirection(WindDirectionInfo::RightToLeft))));

            let line = "info,winddir,tocf";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::WindDirection(WindDirectionInfo::ToCenterField))));

            let line = "info,winddir,tolf";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::WindDirection(WindDirectionInfo::ToLeftField))));

            let line = "info,winddir,torf";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::WindDirection(WindDirectionInfo::ToRightField))));

            let line = "info,winddir,unknown";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::WindDirection(WindDirectionInfo::Unknown))));

            // test invalid
            let line = "info,winddir,a";
            let result = Line::parse(line);
            assert!(result.is_err());
        }

        #[test]
        fn parse_info_windspeed() {
            // test valid
            let line = "info,windspeed,10";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::WindSpeed(WindSpeedInfo::Known(10)))));

            let line = "info,windspeed,-1";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::WindSpeed(WindSpeedInfo::Unknown))));

            // test invalid
            let line = "info,windspeed,a";
            let result = Line::parse(line);
            assert!(result.is_err());
        }

        #[test]
        fn parse_info_timeofgame() {
            // test valid
            let line = "info,timeofgame,120";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::TimeOfGame(TimeOfGameInfo::Known(120)))));

            let line = "info,timeofgame,0";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::TimeOfGame(TimeOfGameInfo::Unknown))));

            // test invalid
            let line = "info,timeofgame,a";
            let result = Line::parse(line);
            assert!(result.is_err());
        }

        #[test]
        fn parse_info_attendance() {
            // test valid
            let line = "info,attendance,120";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Attendance(AttendanceInfo::Known(120)))));

            let line = "info,attendance,0";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Attendance(AttendanceInfo::Unknown))));

            // test invalid
            let line = "info,attendance,a";
            let result = Line::parse(line);
            assert!(result.is_err());
        }

        #[test]
        fn parse_info_site() {
            let line = "info,site,abc";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Site("abc".to_string()))));
        }

        #[test]
        fn parse_info_wp() {
            let line = "info,wp,abc";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::WP("abc".to_string()))));
        }

        #[test]
        fn parse_info_lp() {
            let line = "info,lp,abc";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::LP("abc".to_string()))));
        }
        
        #[test]
        fn parse_info_save() {
            let line = "info,save,abc";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Save(Some("abc".to_string())))));

            let line = "info,save,";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::Save(None))));
        }

        #[test]
        fn parse_info_gwrbi() {
            let line = "info,gwrbi,abc";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::GameWinningRBI(Some("abc".to_string())))));

            let line = "info,gwrbi,";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::GameWinningRBI(None))));
        }

        #[test]
        fn parse_info_gametype() {
            // test valid
            let line = "info,gametype,regular";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::GameType(GameTypeInfo::Regular))));

            let line = "info,gametype,exhibition";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::GameType(GameTypeInfo::Exhibition))));

            let line = "info,gametype,preseason";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::GameType(GameTypeInfo::Preseason))));

            let line = "info,gametype,allstar";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::GameType(GameTypeInfo::AllStar))));

            let line = "info,gametype,playoff";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::GameType(GameTypeInfo::Playoff))));

            let line = "info,gametype,worldseries";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::GameType(GameTypeInfo::WorldSeries))));

            let line = "info,gametype,lcs";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::GameType(GameTypeInfo::LeagueChampionshipSeries))));

            let line = "info,gametype,divisionseries";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::GameType(GameTypeInfo::DivisionSeries))));

            let line = "info,gametype,wildcard";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::GameType(GameTypeInfo::Wildcard))));

            let line = "info,gametype,championship";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Info(Info::GameType(GameTypeInfo::Championship))));

            // test invalid
            let line = "info,gametype,a";
            let result = Line::parse(line);
            assert!(result.is_err());
        }

        #[test]
        fn parse_info_other() {
            let line = "info,key,value";
            let result = Line::parse(line);
            assert_eq!(
                result,
                Ok(Line::Info(Info::Other("key".to_string(), "value".to_string())))
            );
        }

        #[test]
        fn parse_invalid_line_type() {
            let line = "invalid,line";
            let result = Line::parse(line);
            assert!(result.is_err());
        }

        #[test]
        fn parse_start_sub_line() {
            let line = "start,1,player1,0,1,3";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::StartSub {
                is_start: true,
                player_id: "1".to_string(),
                player_name: "player1".to_string(),
                team: Team::Visiting,
                batting_order: 1,
                position: Position::FirstBase,
            }));

            let line = "sub,2,player2,1,2,8";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::StartSub {
                is_start: false,
                player_id: "2".to_string(),
                player_name: "player2".to_string(),
                team: Team::Home,
                batting_order: 2,
                position: Position::CenterField,
            }));
        }
    
        #[test]
        fn parse_play_line() {
            let line = "play,7,0,saboc001,01,CX,8/F78";
            let result = Line::parse(line);
            assert_eq!(result, Ok(Line::Play {
                inning: 7,
                team: Team::Visiting,
                batter_id: "saboc001".to_string(),
                count: Count::Known { balls: 0, strikes: 1 },
                pitches: Some(vec![
                    Pitch {
                        pitch_type: PitchType::CalledStrike,
                        pitch_modifier: None,
                    },
                    Pitch {
                        pitch_type: PitchType::BallInPlayByBatter,
                        pitch_modifier: None,
                    },
                ]),
                event: Event {
                    event_type: EventType::Out {
                        credited_fielder: Fielder::Known(8),
                        assisting_fielders: Vec::new(),
                        runner_out: Runner::Batter,
                    },
                    modifiers: vec![
                        EventModifier::Fly,
                        EventModifier::HitLocation(FieldLocation::CenterLeft),
                    ],
                    advances: Vec::new(),
                },
                note: None,
            }));
        }
    }

    mod game_parser {
        use super::*;

        #[test]
        fn parse_id_line() {
            let mut parser = GameParser::new();

            let line = Line::Id("ANA201004050".to_string());
            let result = parser.parse_line(&line);
            assert!(result.is_ok());
        }

        #[test]
        fn parse_version_line() {
            let mut parser = GameParser::new();

            let line = Line::Version(2);
            let result = parser.parse_line(&line);
            assert!(result.is_ok());
        }

        #[test]
        fn parse_info_line() {
            let mut parser = GameParser::new();

            let line = Line::Info(Info::DayNight(DayNightInfo::Day));
            let result = parser.parse_line(&line);

            assert!(result.is_ok());
            assert!(parser.game_builder.info_state.day_night.is_some());
            assert_eq!(parser.game_builder.info_state.day_night, Some(DayNightInfo::Day));
        }

        #[test]
        fn parse_start_line() {
            let mut parser = GameParser::new();

            let line = Line::Info(Info::HomeTeam("NYA".to_string()));
            let result = parser.parse_line(&line);
            assert!(result.is_ok());

            let line = Line::Info(Info::VisitingTeam("CHC".to_string()));
            let result = parser.parse_line(&line);
            assert!(result.is_ok());

            let line = Line::StartSub {
                is_start: true,
                player_id: "1".to_string(),
                player_name: "player1".to_string(),
                team: Team::Visiting,
                batting_order: 1,
                position: Position::FirstBase,
            };
            let result = parser.parse_line(&line);

            assert!(result.is_ok());
            assert!(parser.game_builder.visiting_team_players.contains(&Player {
                id: "1".to_string(),
                name: "player1".to_string(),
                team: "CHC".to_string(),
                positions: HashSet::from([Position::FirstBase]),
                batting_order: 1,
            }));

            // test multiple positions
            let line = Line::StartSub {
                is_start: true,
                player_id: "2".to_string(),
                player_name: "player2".to_string(),
                team: Team::Home,
                batting_order: 1,
                position: Position::DesignatedHitter,
            };
            let result = parser.parse_line(&line);
            assert!(result.is_ok());
            assert!(parser.game_builder.home_team_players.contains(&Player {
                id: "2".to_string(),
                name: "player2".to_string(),
                team: "NYA".to_string(),
                batting_order: 1,
                positions: HashSet::from([Position::DesignatedHitter]),
            }));

            let line = Line::StartSub {
                is_start: true,
                player_id: "2".to_string(),
                player_name: "player2".to_string(),
                team: Team::Home,
                batting_order: 1,
                position: Position::Pitcher,
            };
            let result = parser.parse_line(&line);
            assert!(result.is_ok());
            assert!(parser.game_builder.home_team_players.contains(&Player {
                id: "2".to_string(),
                name: "player2".to_string(),
                team: "NYA".to_string(),
                batting_order: 1,
                positions: HashSet::from([Position::DesignatedHitter, Position::Pitcher]),
            }));

            assert_eq!(parser.game_builder.home_team_players.len(), 1);
            assert_eq!(parser.game_builder.visiting_team_players.len(), 1);
        }
    }
}
